# -*- coding: utf-8 -*-
"""Order ties inventory (T-049, input for P3b). Read-only: reads the atlas JSON and the *text* of the
kit's `module_*.py` files; it never imports or runs kit Python.

    python py/order_ties.py [--atlas PATH] [--kit DIR] [--out PATH]

What it counts
  1. ranges: every `*_begin` / `*_end` constant of the atlas, with its value resolved to a table index
     (string values are record ids -> (table, index); numbers stay numbers and get no table), the ranges
     per table, and the chains `X_end == Y_begin` (an `_end` and a `_begin` that resolve to the same
     (table, index)).
  2. offset arithmetic sites: every operation call in the kit's module files whose op is an arithmetic op
     of the atlas (`store_add`, `store_sub`, `val_add`, `val_sub`, ...) AND whose arguments name a record
     id or a range constant. Sites are grouped by table and by range constant.

Prints a summary (the numbers a document may quote) and writes `target/uw-native/order_ties.json`.
Exit code 0 when the atlas resolves every input it needs; 1 when the atlas is missing/unusable.
"""
import argparse
import io
import json
import os
import re
import sys

# Arithmetic ops of the atlas that do a computation. A name is only used when the atlas has that op
# (nothing is guessed): the set is filtered against atlas["ops"] below.
ARITH_RE = re.compile(r"^(?:store|val)_(?:add|sub|mul|div|mod|or|and|xor|min|max|neg|abs|clamp|sqrt|pow)$")
# "( <op> ," at a call position.
CALL_RE = re.compile(r"\s*([A-Za-z_]\w*)\s*,")
# A bare identifier argument.
IDENT_RE = re.compile(r"^[A-Za-z_]\w*$")


def read_text(path):
    with io.open(path, "r", encoding="latin-1") as fh:
        return fh.read()


def load_atlas(path):
    with io.open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def build_index(atlas):
    """id -> (table, index) from the atlas tables, plus the table list in atlas order."""
    ids = {}
    tables = []
    for table_name in sorted(atlas.get("tables") or {}):
        table = atlas["tables"][table_name]
        tables.append((table_name, table))
        for entry in table.get("entries") or []:
            key = entry.get("id")
            if not key:
                continue
            ids.setdefault(key, []).append((table_name, entry.get("index")))
    return ids, tables


def unique_id(ids, name):
    """(table, index) when `name` is one record id of the atlas; None when unknown or ambiguous."""
    hits = ids.get(name) or []
    if len(hits) != 1:
        return None
    return hits[0]


def analyse_ranges(atlas, ids):
    """Resolve every range name to a table index and find the `X_end == Y_begin` chains."""
    definitions = atlas.get("ranges") or []
    lines = {}
    for item in definitions:
        lines.setdefault(item.get("name"), []).append(item.get("line"))
    # The kit assigns a name once effectively: last assignment wins (module_constants.py semantics).
    last = {}
    order = []
    for item in definitions:
        name = item.get("name")
        if name not in last:
            order.append(name)
        last[name] = item

    items = []
    for name in order:
        raw = last[name]
        value = raw.get("value")
        item = {
            "name": name,
            "value": value,
            "line": raw.get("line"),
            "lines": lines.get(name) or [],
            "kind": None,
            "table": None,
            "index": None,
        }
        if isinstance(value, bool):
            item["kind"] = "unresolved"
            item["reason"] = "boolean value"
        elif isinstance(value, int):
            item["kind"] = "number"
            item["index"] = value
        elif isinstance(value, str):
            hit = unique_id(ids, value)
            if hit is None:
                item["kind"] = "unresolved"
                item["reason"] = "value %r is not a unique record id" % (value,)
            else:
                item["kind"] = "id"
                item["table"], item["index"] = hit
        else:
            item["kind"] = "unresolved"
            item["reason"] = "value type %s" % (type(value).__name__,)
        items.append(item)

    # Chains: an _end and a _begin that resolve to the same (table, index).
    begins = {}
    ends = {}
    for item in items:
        if item["kind"] != "id":
            continue
        key = (item["table"], item["index"])
        if item["name"].endswith("_begin"):
            begins.setdefault(key, []).append(item)
        elif item["name"].endswith("_end"):
            ends.setdefault(key, []).append(item)
    chains = []
    for key in sorted(ends):
        for end in ends[key]:
            for begin in begins.get(key) or []:
                chains.append({
                    "end": end["name"],
                    "begin": begin["name"],
                    "table": key[0],
                    "index": key[1],
                    "end_line": end["line"],
                    "begin_line": begin["line"],
                })

    # Ranges per table.
    by_table = {}
    for item in items:
        if item["kind"] != "id":
            continue
        slot = by_table.setdefault(item["table"], {"begin": [], "end": [], "other": []})
        if item["name"].endswith("_begin"):
            slot["begin"].append(item["name"])
        elif item["name"].endswith("_end"):
            slot["end"].append(item["name"])
        else:
            slot["other"].append(item["name"])

    resolved = [item for item in items if item["kind"] == "id"]
    numeric = [item for item in items if item["kind"] == "number"]
    unresolved = [item for item in items if item["kind"] == "unresolved"]
    return {
        "definitions": len(definitions),
        "total": len(items),
        "resolved": len(resolved),
        "numeric": len(numeric),
        "unresolved": unresolved,
        "by_table": by_table,
        "items": items,
        "chains": chains,
        "chain_count": len(chains),
    }


def skip_string(text, pos):
    """Index just past the string literal that starts at `pos` (quote-aware, backslash escapes)."""
    quote = text[pos]
    i = pos + 1
    n = len(text)
    while i < n:
        c = text[i]
        if c == "\\":
            i += 2
            continue
        if c == quote:
            return i + 1
        i += 1
    return n


def cut_comment(arg):
    """`arg` without a trailing `#` comment (a `#` inside a string is not a comment)."""
    i = 0
    n = len(arg)
    while i < n:
        c = arg[i]
        if c in "\"'":
            i = skip_string(arg, i)
            continue
        if c == "#":
            return arg[:i]
        i += 1
    return arg


def read_args(text, pos):
    """Arguments of the call whose first argument starts at `pos`; returns (args, end index)."""
    args = []
    depth = 0
    start = pos
    i = pos
    n = len(text)
    while i < n:
        c = text[i]
        if c in "\"'":
            i = skip_string(text, i)
            continue
        if c == "#":
            end = text.find("\n", i)
            if end < 0:
                i = n
                continue
            i = end
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            if depth == 0:
                args.append(text[start:i])
                return args, i + 1
            depth -= 1
        elif c == "," and depth == 0:
            args.append(text[start:i])
            start = i + 1
        i += 1
    args.append(text[start:n])
    return args, n


def arg_token(arg):
    """The name an argument carries: the literal of a string, or a bare identifier; else None."""
    token = cut_comment(arg).strip()
    if not token:
        return None
    if token[0] in "\"'":
        end = skip_string(token, 0)
        if end != len(token):
            return None  # implicit concatenation / escape: not a plain name
        return token[1:-1]
    if IDENT_RE.match(token):
        return token
    return None


def scan_file(path, rel, arith_ops, ids, ranges):
    """Offset arithmetic sites of one module file: (file, line, op, kind, token, table, index)."""
    text = read_text(path)
    sites = []
    unknown_ops = []
    n = len(text)
    i = 0
    line = 1
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1
            i += 1
            continue
        if c == "#":
            end = text.find("\n", i)
            if end < 0:
                break
            i = end
            continue
        if c in "\"'":
            end = skip_string(text, i)
            line += text.count("\n", i, end)
            i = end
            continue
        if c == "(":
            match = CALL_RE.match(text, i + 1)
            if match:
                op = match.group(1)
                is_arith = op in arith_ops
                if is_arith or ARITH_RE.match(op):
                    args, end = read_args(text, match.end())
                    if not is_arith and op not in unknown_ops:
                        unknown_ops.append(op)
                    for arg in args:
                        token = arg_token(arg)
                        if token is None:
                            continue
                        hit = unique_id(ids, token)
                        if hit is not None:
                            sites.append({"file": rel, "line": line, "op": op, "kind": "id",
                                          "token": token, "table": hit[0], "index": hit[1]})
                            continue
                        item = ranges.get(token)
                        if item is not None:
                            sites.append({"file": rel, "line": line, "op": op, "kind": "range",
                                          "token": token, "table": item["table"],
                                          "index": item["index"]})
                    line += text.count("\n", i, end)
                    i = end
                    continue
        i += 1
    return sites, unknown_ops


def collect_sites(atlas, ids, ranges, kit):
    arith_ops = set(op.get("name") for op in atlas.get("ops") or []
                    if ARITH_RE.match(op.get("name") or ""))
    modules = []
    for table_name in sorted(atlas.get("tables") or {}):
        module = atlas["tables"][table_name].get("module")
        if module and module not in modules:
            modules.append(module)
    sites = []
    unknown_ops = []
    missing = []
    files = []
    for module in modules:
        name = module + ".py"
        path = os.path.join(kit, name)
        if not os.path.isfile(path):
            missing.append(name)
            continue
        files.append(name)
        found, unknown = scan_file(path, name, arith_ops, ids, ranges)
        sites.extend(found)
        for op in unknown:
            if op not in unknown_ops:
                unknown_ops.append(op)

    by_kind = {"id": 0, "range": 0}
    by_table = {}
    by_constant = {}
    for site in sites:
        by_kind[site["kind"]] += 1
        if site["table"]:
            by_table[site["table"]] = by_table.get(site["table"], 0) + 1
        if site["kind"] == "range":
            by_constant[site["token"]] = by_constant.get(site["token"], 0) + 1
    return {
        "files": files,
        "missing_files": missing,
        "arith_ops": sorted(arith_ops),
        "unknown_ops": unknown_ops,
        "total": len(sites),
        "by_kind": by_kind,
        "by_table": by_table,
        "by_constant": by_constant,
        "sites": sites,
    }


def top(counts, limit=8):
    pairs = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))
    return pairs[:limit]


def main(argv=None):
    here = os.path.dirname(os.path.abspath(__file__))
    root = os.path.dirname(here)
    parser = argparse.ArgumentParser(description="Order ties inventory (T-049): ranges + id offset sites.")
    parser.add_argument("--atlas", default=os.path.join(root, "target", "uw-staging", "atlas.json"),
                        help="atlas JSON (default: target/uw-staging/atlas.json)")
    parser.add_argument("--kit", default=os.path.join(root, "Module_system 1.171"),
                        help="read-only kit folder (default: Module_system 1.171)")
    parser.add_argument("--out", default=os.path.join(root, "target", "uw-native", "order_ties.json"),
                        help="output JSON (default: target/uw-native/order_ties.json)")
    opts = parser.parse_args(argv)

    if not os.path.isfile(opts.atlas):
        sys.stderr.write("atlas not found: %s\n" % (opts.atlas,))
        return 1
    if not os.path.isdir(opts.kit):
        sys.stderr.write("kit folder not found: %s\n" % (opts.kit,))
        return 1

    atlas = load_atlas(opts.atlas)
    ids, _tables = build_index(atlas)
    ranges = analyse_ranges(atlas, ids)
    range_by_name = {}
    for item in ranges["items"]:
        range_by_name[item["name"]] = item
    sites = collect_sites(atlas, ids, range_by_name, opts.kit)

    result = {
        "atlas": {"path": opts.atlas, "kit_hash": atlas.get("kit_hash"), "schema": atlas.get("schema")},
        "kit": opts.kit,
        "ranges": ranges,
        "offset_sites": sites,
    }
    out_dir = os.path.dirname(opts.out)
    if out_dir and not os.path.isdir(out_dir):
        os.makedirs(out_dir)
    with io.open(opts.out, "w", encoding="utf-8") as fh:
        fh.write(json.dumps(result, ensure_ascii=False, indent=1, sort_keys=False))
        fh.write("\n")

    # Summary (the numbers a document may quote).
    print("atlas: %s (schema %s, kit_hash %s)" % (opts.atlas, atlas.get("schema"), atlas.get("kit_hash")))
    print("ranges: %d names (%d definitions); resolved to a table index: %d; numeric: %d; unresolved: %d"
          % (ranges["total"], ranges["definitions"], ranges["resolved"], ranges["numeric"],
             len(ranges["unresolved"])))
    print("ranges per table: %d tables (%s)" % (
        len(ranges["by_table"]),
        ", ".join("%s %d" % (name, len(slot["begin"]) + len(slot["end"]) + len(slot["other"]))
                  for name, slot in sorted(ranges["by_table"].items()))))
    print("chains (X_end == Y_begin): %d" % (ranges["chain_count"],))
    for chain in ranges["chains"]:
        print("  chain %s == %s (%s #%d)"
              % (chain["end"], chain["begin"], chain["table"], chain["index"]))
    for item in ranges["unresolved"]:
        print("  unresolved range %s (line %s): %s"
              % (item["name"], item["line"], item.get("reason")))
    print("arith ops from the atlas: %s" % (", ".join(sites["arith_ops"]),))
    if sites["unknown_ops"]:
        print("WARNING arithmetic-looking ops missing from the atlas: %s" % (", ".join(sites["unknown_ops"]),))
    if sites["missing_files"]:
        print("WARNING module files missing in the kit: %s" % (", ".join(sites["missing_files"]),))
    print("offset sites: %d in %d files (record id: %d, range constant: %d)"
          % (sites["total"], len(sites["files"]), sites["by_kind"]["id"], sites["by_kind"]["range"]))
    print("  by table: %s" % (", ".join("%s %d" % (name, count) for name, count in top(sites["by_table"])),))
    print("  distinct constants: %d" % (len(sites["by_constant"]),))
    print("  by constant: %s"
          % (", ".join("%s %d" % (name, count) for name, count in top(sites["by_constant"])),))
    print("wrote %s" % (opts.out,))
    return 0


if __name__ == "__main__":
    sys.exit(main())
