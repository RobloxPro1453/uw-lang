# -*- coding: utf-8 -*-
"""uw_runtime: the helpers UW's generated `module_X.py` wrappers call (Python 2.7, runs inside the
kit staging copy; also importable by Python 3 for tests). MIT OR Apache-2.0.

A wrapper looks like:

    from native_module_scripts import *
    import uw_runtime as uw
    scripts = uw.append_ops(scripts, "game_start", 1, [(assign, reg0, reg0)])

Rules (docs/ARCHITECTURE.md > Kopyalama siniri, KARARNAMELER 13):
  * Nothing is changed in place. Every helper returns a NEW list: the list itself is a shallow
    copy, the one entry being changed is a deep copy, every other entry is the very same object.
    Objects Native shares between entries (one trigger tuple in 21 mission templates) therefore
    stay shared and unchanged for every entry the patch does not name.
  * Entries are found by name the way the kit's ID writer names them: the kit's own
    `convert_to_identifier(name)` (lower case). A name that repeats must be written `name#n`
    (1-based occurrence in list order); a bare repeated name is an error, never a guess.
  * Tuples stay tuples and lists stay lists (the kit tells them apart).
  * A path is a field index or a tuple of indices from the entry down to an operation block,
    e.g. 1 for a script's body, (5, 3, 4) for the consequences of trigger 3 of a mission template.
"""
import copy


class UwRuntimeError(Exception):
    pass


def _identifier(name):
    try:
        from process_common import convert_to_identifier
    except ImportError:  # outside the kit (unit tests): only reached when no kit is importable
        raise UwRuntimeError("process_common (the kit) is not importable")
    return convert_to_identifier(name).lower()


def find(lst, ref, key=None):
    """Index of the entry `ref` ("name" or "name#n") in a named table list."""
    key = key or _identifier
    name, _, occ = ref.partition("#")
    wanted = key(name)
    hits = [i for i, e in enumerate(lst) if key(e[0]) == wanted]
    if not hits:
        raise UwRuntimeError("no entry named %r" % name)
    if occ:
        try:
            n = int(occ)
        except ValueError:
            raise UwRuntimeError("bad occurrence in %r" % ref)
        if not 1 <= n <= len(hits):
            raise UwRuntimeError("%r: there are %d entries named %r" % (ref, len(hits), name))
        return hits[n - 1]
    if len(hits) > 1:
        raise UwRuntimeError("%r names %d entries; write %s#1 .. %s#%d"
                             % (name, len(hits), name, name, len(hits)))
    return hits[0]


def _with(lst, index, new_entry):
    out = list(lst)
    out[index] = new_entry
    return out


def _rebuild(seq, items):
    """Same container type as `seq` (tuple or list) with `items`."""
    return tuple(items) if isinstance(seq, tuple) else list(items)


def _path(path):
    return (path,) if isinstance(path, int) else tuple(path)


def _set_in(node, path, fn):
    """Copy of `node` with fn(old) placed at `path`; containers on the path are rebuilt."""
    if not path:
        return fn(node)
    head, rest = path[0], path[1:]
    if not isinstance(node, (list, tuple)) or not -len(node) <= head < len(node):
        raise UwRuntimeError("path index %r does not exist" % (head,))
    items = list(node)
    items[head] = _set_in(items[head], rest, fn)
    return _rebuild(node, items)


def _block(old, ops, how):
    if not isinstance(old, list):
        raise UwRuntimeError("path does not end at an operation block (a list), found %s"
                             % type(old).__name__)
    ops = copy.deepcopy(list(ops))
    if how == "prepend":
        return ops + old
    if how == "append":
        return old + ops
    return ops


def _patch(lst, ref, path, fn, key):
    i = find(lst, ref, key)
    entry = copy.deepcopy(lst[i])
    return _with(lst, i, _set_in(entry, _path(path), fn))


def replace_entry(lst, ref, new_entry, key=None):
    """Whole entry replaced (a deep copy of `new_entry`)."""
    return _with(lst, find(lst, ref, key), copy.deepcopy(new_entry))


def replace_field(lst, ref, path, value, key=None):
    """One field (any depth) of the entry set to a deep copy of `value`."""
    return _patch(lst, ref, path, lambda _old: copy.deepcopy(value), key)


def replace_body(lst, ref, path, ops, key=None):
    return _patch(lst, ref, path, lambda old: _block(old, ops, "replace"), key)


def prepend_ops(lst, ref, path, ops, key=None):
    return _patch(lst, ref, path, lambda old: _block(old, ops, "prepend"), key)


def append_ops(lst, ref, path, ops, key=None):
    return _patch(lst, ref, path, lambda old: _block(old, ops, "append"), key)


def extend(lst, entries, key=None):
    """New entries at the end. A name already in the list is an error (KARARNAMELER 12)."""
    key = key or _identifier
    taken = set(key(e[0]) for e in lst)
    out = list(lst)
    for e in entries:
        k = key(e[0])
        if k in taken:
            raise UwRuntimeError("new entry %r clashes with an existing name" % e[0])
        taken.add(k)
        out.append(copy.deepcopy(e))
    return out


def tag_all(lst, path, make_ops):
    """Debug builds only: every entry whose `path` is an operation block gets make_ops(entry) in
    front of it (deep copy of that entry); entries without such a block stay the same object."""
    out = []
    for e in lst:
        try:
            node = e
            for i in _path(path):
                node = node[i]
        except (IndexError, TypeError, KeyError):
            out.append(e)
            continue
        if not isinstance(node, list):
            out.append(e)
            continue
        ops = make_ops(e)
        out.append(_set_in(copy.deepcopy(e), _path(path), lambda old: _block(old, ops, "prepend")))
    return out


# Unnamed tables (dialogs, triggers, simple_triggers): the plan gives list indices.

def insert_before(lst, index, entries):
    if not 0 <= index <= len(lst):
        raise UwRuntimeError("index %d out of range" % index)
    return list(lst[:index]) + copy.deepcopy(list(entries)) + list(lst[index:])


def insert_after(lst, index, entries):
    if not 0 <= index < len(lst):
        raise UwRuntimeError("index %d out of range" % index)
    return insert_before(lst, index + 1, entries)


def remove_where(lst, predicate):
    """Entries for which predicate(index, entry) is true are left out."""
    return [e for i, e in enumerate(lst) if not predicate(i, e)]
