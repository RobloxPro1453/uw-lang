# -*- coding: utf-8 -*-
"""Atlas dump (Faz 0 prototype). Python 2.7 ONLY, run with -B -E -s inside an *atlas staging* copy of
the kit (the kit's own ID_*.py present), never inside the kit folder itself:

    C:/Python27/python.exe -B -E -s <repo>/py/atlas_dump.py <out.json> [kit_hash]

Every name, index and value here is read from the kit; nothing is typed in by hand except the
table list below, and each table is checked against its ID_*.py file (same length, same order,
duplicate names resolved "last wins" exactly like the kit's ID writer).

Output JSON (schema 1):
  ops:     [{name, code, line, signature, lhs, can_fail, global_lhs}]   from header_operations.py
  hooks:   ["game_start", ...]                                          scripts named game_*
  constants: [{name, value, module, line}]                              ints of header_*.py (not ops) + module_constants.py
  ranges:  [{name, value, line}]                                        *_begin / *_end in module_constants.py
  tables:  {table: {module, var, id_prefix, id_file, ref_tag, entries: [{index, name, id, dup, line, value}]}}
           ref_tag: the tag operations use for this table ("mnu" for menus), from process_operations.py;
           null when operations cannot name it. "<ref_tag>_<name>" resolves to the FIRST entry whose
           name matches case-insensitively (header_common.find_object), unlike ID_*.py (last wins).
           unnamed tables (dialogs, triggers, simple_triggers) have name = null; dialogs get the kit's
           own `dlga_<in>:<out>[.N]` id from a previous build's conversation.txt (else null), and the
           line of their text literal.
           value: a Python tuple is a JSON array, a Python list is {"list": [...]} (the kit cares).
Source lines are read-only pointers for `uw explain`; output generation never uses source text.
"""
import io
import json
import re
import sys

# table: (module, list variable, ID file or None, id prefix or None). Verified against the kit
# (process_*.py imports, ID_*.py first lines) on 2026-11-15.
TABLES = [
    ("animations", "module_animations", "animations", "ID_animations", "anim_"),
    ("factions", "module_factions", "factions", "ID_factions", "fac_"),
    ("info_pages", "module_info_pages", "info_pages", "ID_info_pages", "ip_"),
    ("items", "module_items", "items", "ID_items", "itm_"),
    ("map_icons", "module_map_icons", "map_icons", "ID_map_icons", "icon_"),
    ("menus", "module_game_menus", "game_menus", "ID_menus", "menu_"),
    ("meshes", "module_meshes", "meshes", "ID_meshes", "mesh_"),
    ("mission_templates", "module_mission_templates", "mission_templates", "ID_mission_templates", "mst_"),
    ("music", "module_music", "tracks", "ID_music", "track_"),
    ("particle_systems", "module_particle_systems", "particle_systems", "ID_particle_systems", "psys_"),
    ("parties", "module_parties", "parties", "ID_parties", "p_"),
    ("party_templates", "module_party_templates", "party_templates", "ID_party_templates", "pt_"),
    ("postfx_params", "module_postfx", "postfx_params", "ID_postfx_params", "pfx_"),
    ("presentations", "module_presentations", "presentations", "ID_presentations", "prsnt_"),
    ("quests", "module_quests", "quests", "ID_quests", "qst_"),
    ("scene_props", "module_scene_props", "scene_props", "ID_scene_props", "spr_"),
    ("scenes", "module_scenes", "scenes", "ID_scenes", "scn_"),
    ("scripts", "module_scripts", "scripts", "ID_scripts", "script_"),
    ("skills", "module_skills", "skills", "ID_skills", "skl_"),
    ("sounds", "module_sounds", "sounds", "ID_sounds", "snd_"),
    ("strings", "module_strings", "strings", "ID_strings", "str_"),
    ("tableau_materials", "module_tableau_materials", "tableaus", "ID_tableau_materials", "tableau_"),
    ("troops", "module_troops", "troops", "ID_troops", "trp_"),
    ("skins", "module_skins", "skins", None, None),
    ("dialogs", "module_dialogs", "dialogs", None, None),
    ("triggers", "module_triggers", "triggers", None, None),
    ("simple_triggers", "module_simple_triggers", "simple_triggers", None, None),
]

ASSIGN_RE = re.compile(r"^([A-Za-z_]\w*)\s*=\s*(0x[0-9a-fA-F]+|\d+)\b(.*)$")


def read_lines(path):
    with io.open(path, "r", encoding="latin-1") as fh:
        return fh.read().splitlines()


def jsonable(v):
    """Entry values. The kit tells tuples from lists (D3: `"%f" % [..]` fails), so both survive:
    tuple -> JSON array, list -> {"list": [...]}. Longs stay ints; byte strings decoded as latin-1."""
    if isinstance(v, tuple):
        return [jsonable(x) for x in v]
    if isinstance(v, list):
        return {"list": [jsonable(x) for x in v]}
    if isinstance(v, str):
        return v.decode("latin-1")
    if isinstance(v, (int, long, float, bool)) or v is None:
        return v
    if isinstance(v, unicode):
        return v
    raise TypeError("unexpected value type %r" % type(v))


def dump_ops():
    import header_operations as ho
    lhs = set(ho.lhs_operations)
    can_fail = set(ho.can_fail_operations)
    glhs = set(ho.global_lhs_operations)
    ops = []
    lines = read_lines("header_operations.py")
    for no, line in enumerate(lines, 1):
        m = ASSIGN_RE.match(line)
        if not m:
            continue
        name = m.group(1)
        code = getattr(ho, name)
        rest = m.group(3).strip()
        sig = rest[1:].strip() if rest.startswith("#") else ""
        # Some ops document their call form on the comment-only line(s) below the assignment
        # (store_agent_hit_points: "# set absolute to 1 ..." then "#  (store_agent_hit_points,<destination>,...)").
        if ("(" + name) not in sig.replace(" ", ""):
            for follow in lines[no:no + 3]:
                text = follow.strip()
                if not text.startswith("#"):
                    break
                if ("(" + name) in text.replace(" ", ""):
                    sig = (sig + " # " + text[1:].strip()).strip()
                    break
        ops.append({"name": name, "code": code, "line": no, "signature": sig,
                    "lhs": code in lhs, "can_fail": code in can_fail, "global_lhs": code in glhs})
    return ops


def dump_ranges():
    out = []
    for no, line in enumerate(read_lines("module_constants.py"), 1):
        m = re.match(r"^([A-Za-z_]\w*_(?:begin|end))\s*=", line)
        if m:
            import module_constants as mc
            out.append({"name": m.group(1), "value": getattr(mc, m.group(1)), "line": no})
    return out


CONST_RE = re.compile(r"^([A-Za-z_]\w*)\s*=")


def dump_constants():
    """Every module-level integer a UW call may pass by name: header_*.py (except
    header_operations.py, whose names are the ops) and module_constants.py. [{name, value, module,
    line}], in file then line order; a name assigned twice in a file keeps its last line."""
    import glob
    out = []
    files = sorted(f for f in glob.glob("header_*.py") if f != "header_operations.py")
    for path in files + ["module_constants.py"]:
        mod = __import__(path[:-3])
        seen = {}
        for no, line in enumerate(read_lines(path), 1):
            m = CONST_RE.match(line)
            if m and isinstance(getattr(mod, m.group(1), None), (int, long))                     and not isinstance(getattr(mod, m.group(1)), bool):
                seen[m.group(1)] = no
            # Trigger kinds (`ti_on_agent_spawn = -20.0`, header_triggers.py) are floats; keep them so
            # tools can list every hook (uw coverage hook grid).
            elif m and m.group(1).startswith("ti_") and isinstance(getattr(mod, m.group(1), None), float):
                seen[m.group(1)] = no
        for name, no in sorted(seen.items(), key=lambda kv: kv[1]):
            out.append({"name": name, "value": getattr(mod, name), "module": path[:-3], "line": no})
    return out


def read_id_file(id_file):
    pairs = []
    # The kit writes "<prefix><identifier> = <index>". An identifier can itself contain " = "
    # (17 Native strings do; Python then reads a chained assignment), so split on the LAST one.
    for line in read_lines(id_file + ".py"):
        if " = " not in line:
            continue
        ident, idx = line.rsplit(" = ", 1)
        if idx.strip().isdigit():
            pairs.append((ident, int(idx)))
    return pairs


def entry_lines(module, names):
    """Line of each named entry's opening `("name",` / `["name",` (the bracket may sit on the line
    before) in the module source, searched in list order. None when not found. Pointer only."""
    with io.open(module + ".py", "r", encoding="latin-1") as fh:
        text = fh.read()
    out, pos = [], 0
    for name in names:
        m = re.compile(r'[\(\[]\s*"%s"\s*,' % re.escape(name)).search(text, pos)
        if m is None:
            out.append(None)
            continue
        start = text.index('"', m.start())
        out.append(text.count("\n", 0, start) + 1)
        pos = m.end()
    return out


def dialog_lines(module, entries):
    """Line of each dialog's text literal (field 4, module_dialogs.py header comment), searched in
    list order. None when the literal is not written plainly in the source. Pointer only."""
    with io.open(module + ".py", "r", encoding="latin-1") as fh:
        text = fh.read()
    out, pos = [], 0
    for e in entries:
        lit = e[3] if len(e) > 3 and isinstance(e[3], str) else None
        found = text.find('"%s"' % lit, pos) if lit else -1
        if found < 0:
            out.append(None)
            continue
        out.append(text.count("\n", 0, found) + 1)
        pos = found + len(lit) + 2
    return out


def dialog_ids(count):
    """The kit's own dialog ids (`dlga_<in>:<out>[.N]`, process_dialogs.create_auto_id2), read from
    the conversation.txt a kit build left in export_dir, one per dialog in list order. All None
    when there is no such file or its count does not match."""
    import os
    from module_info import export_dir
    path = os.path.join(export_dir, "conversation.txt")
    if not os.path.isfile(path):
        return [None] * count
    with io.open(path, "r", encoding="latin-1") as fh:
        rows = fh.read().splitlines()
    ids = [r.split(" ", 1)[0] for r in rows[2:] if r.startswith("dlga_")]
    return ids if len(ids) == count else [None] * count


def dump_table(table, module, var, id_file, prefix):
    mod = __import__(module)
    entries = getattr(mod, var)
    named = id_file is not None
    names = [e[0] for e in entries] if named else [None] * len(entries)
    ids_by_index = {}
    if named:
        from process_common import convert_to_identifier
        # The kit's ID file has one line per entry, in list order; when two entries share an
        # identifier, Python keeps the LAST assignment, so the earlier entry has no usable ID.
        lines_ = [p for p in read_id_file(id_file) if p[0].startswith(prefix)]
        if [p[1] for p in lines_] != range(len(entries)):
            raise SystemExit("%s: ID file is not one line per entry in order" % table)
        last = dict(lines_)
        for i, (ident, _idx) in enumerate(lines_):
            # Cross-check with the kit's own identifier function (case aside: some writers keep case).
            if ident[len(prefix):].lower() != convert_to_identifier(names[i]).lower():
                raise SystemExit("%s[%d]: ID %r does not match name %r" % (table, i, ident, names[i]))
            ids_by_index[i] = ident if last[ident] == i else None
    keys = [None] * len(entries)
    if named:
        keys = [convert_to_identifier(n).lower() for n in names]
    counts, seen = {}, {}
    for k in keys:
        if k is not None:
            counts[k] = counts.get(k, 0) + 1
    if named:
        lines = entry_lines(module, names)
    elif table == "dialogs":
        lines = dialog_lines(module, entries)
        ids_by_index = dict(enumerate(dialog_ids(len(entries))))
    else:
        lines = [None] * len(entries)
    out = []
    for i, e in enumerate(entries):
        k = keys[i]
        dup = None  # 1-based occurrence, only for identifiers that really repeat (`ad#n`)
        if k is not None and counts[k] > 1:
            seen[k] = seen.get(k, 0) + 1
            dup = seen[k]
        out.append({"index": i, "name": names[i], "id": ids_by_index.get(i),
                    "dup": dup, "line": lines[i], "value": jsonable(e)})
    return {"module": module, "var": var, "id_file": id_file, "id_prefix": prefix, "entries": out}


TAG_RE = re.compile(r'tag\s*==\s*"(\w+)"')
FIND_RE = re.compile(r"find_object\((\w+)\s*,")


def dump_ref_tags():
    """{list variable: tag} as the kit's own `get_id_value` in process_operations.py resolves
    "tag_name" strings inside operations (e.g. "mnu" -> game_menus, "mt" -> mission_templates,
    NOT the ID_*.py prefixes). Read from the source, nothing typed in by hand."""
    tags, pending = {}, None
    for line in read_lines("process_operations.py"):
        m = TAG_RE.search(line)
        if m:
            pending = m.group(1)
            continue
        m = FIND_RE.search(line)
        if m and pending is not None:
            tags[m.group(1)] = pending
            pending = None
    if not tags:
        raise SystemExit("process_operations.py: no tag table found")
    return tags


def main():
    import os
    sys.path.insert(0, os.getcwd())  # the staging copy, not this script's folder
    out_path = sys.argv[1]
    kit_hash = sys.argv[2] if len(sys.argv) > 2 else None
    ref_tags = dump_ref_tags()
    tables = {}
    for t in TABLES:
        tables[t[0]] = dump_table(*t)
        tables[t[0]]["ref_tag"] = ref_tags.get(t[2])
    hooks = [e["name"] for e in tables["scripts"]["entries"] if e["name"].startswith("game_")]
    atlas = {"schema": 1, "kit_hash": kit_hash, "ops": dump_ops(), "hooks": hooks,
             "ranges": dump_ranges(), "constants": dump_constants(), "tables": tables}
    with io.open(out_path, "w", encoding="utf-8") as fh:
        fh.write(unicode(json.dumps(atlas, ensure_ascii=False, sort_keys=True, separators=(",", ":"))))
    print("ops %d, hooks %d, ranges %d, tables %d, entries %d" % (
        len(atlas["ops"]), len(hooks), len(atlas["ranges"]), len(tables),
        sum(len(t["entries"]) for t in tables.values())))


if __name__ == "__main__":
    main()
