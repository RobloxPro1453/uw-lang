#!/usr/bin/env python3
"""Exports Native's menu flow as JSON for the graph_viewer project.

Reads only the atlas (target/uw-staging/atlas.json). Every menu becomes a node with its text and its
options (each option: id, text, whether it has a condition, and where its consequences go); edges
come from operations inside menu bodies and options: jump_to_menu (to a menu), change_screen_*
(to a screen), call_script / start_presentation / jump_to_scene / set_jump_mission (to that thing).
Operation codes are taken from the atlas' own op list, never typed in.

Superseded by `uw graph --json flow.json` (crates/uw_cli/src/flow_json.rs), which writes the identical
file; kept as a cross-check.

Usage: python py/flow_export.py [--atlas target/uw-staging/atlas.json] [--out flow.json]
"""
import argparse
import json

EDGE_OPS = ["jump_to_menu", "call_script", "start_presentation", "jump_to_scene", "set_jump_mission"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--atlas", default="target/uw-staging/atlas.json")
    ap.add_argument("--out", default="flow.json")
    a = ap.parse_args()
    atlas = json.load(open(a.atlas, encoding="utf-8"))
    ops = {}
    for op in atlas["ops"]:
        ops.setdefault(op["code"], op["name"])
    flags = sum(op["code"] for op in atlas["ops"] if op["name"] in ("neg", "this_or_next"))
    tags = {t["ref_tag"]: (name, t) for name, t in atlas["tables"].items() if t.get("ref_tag")}

    def op_name(code):
        return ops.get(code) or ops.get(code & ~flags)

    def resolve(text):
        """'mnu_x' -> 'menu_x' style label, as the kit resolves it (first entry with that name)."""
        if not isinstance(text, str) or "_" not in text:
            return None
        tag, rest = text.split("_", 1)
        if tag not in tags:
            return None
        table_name, table = tags[tag]
        for e in table["entries"]:
            if (e["name"] or "").lower() == rest.lower():
                return e["id"] or "%s%s" % (table["id_prefix"], e["name"].lower())
        return None

    def block(value):
        return value["list"] if isinstance(value, dict) and "list" in value else (value or [])

    def edges_of(ops_block):
        out = []
        for st in block(ops_block):
            if not (isinstance(st, list) and st and isinstance(st[0], int)):
                continue
            name = op_name(st[0])
            if name and name.startswith("change_screen_"):
                out.append({"op": name, "to": "screen:" + name[len("change_screen_"):]})
            elif name in EDGE_OPS and len(st) > 1:
                target = resolve(st[1])
                if target:
                    out.append({"op": name, "to": target})
        return out

    menus = atlas["tables"]["menus"]["entries"]
    nodes, edges = [], []
    for e in menus:
        v = block(e["value"])
        label = e["id"] or "menu_%s#%s" % (e["name"].lower(), e["dup"])
        options = []
        for opt in block(v[5]) if len(v) > 5 else []:
            opt = block(opt)
            if len(opt) < 4:
                continue
            option_edges = edges_of(opt[3])
            options.append({
                "index": len(options),
                "id": "mno_" + str(opt[0]),
                "text": opt[2],
                "conditional": bool(block(opt[1])),
                "goes_to": option_edges,
            })
            for ed in option_edges:
                edges.append({"from": label, "to": ed["to"], "op": ed["op"], "via_option": "mno_" + str(opt[0]),
                              "option_index": len(options) - 1})
        on_enter = edges_of(v[4]) if len(v) > 4 else []
        for ed in on_enter:
            edges.append({"from": label, "to": ed["to"], "op": ed["op"], "via_option": None,
                          "option_index": None})
        nodes.append({
            "id": label,
            "kind": "menu",
            "index": e["index"],
            "text": v[2] if len(v) > 2 else "",
            "source": "module_game_menus.py:%s" % e["line"] if e["line"] else "module_game_menus.py",
            "on_enter_goes_to": on_enter,
            "options": options,
            "stub": False,
        })
    # Every arrow target gets a node: things that are not menus become small "stub" nodes.
    KINDS = {"script": "script", "mst": "mission_template", "scn": "scene", "prsnt": "presentation",
             "screen": "screen"}
    known = set(n["id"] for n in nodes)
    for ed in edges:
        t = ed["to"]
        if t in known:
            continue
        known.add(t)
        prefix = "screen" if t.startswith("screen:") else t.split("_", 1)[0]
        nodes.append({"id": t, "kind": KINDS.get(prefix, "other"), "index": -1, "text": "",
                      "source": "", "on_enter_goes_to": [], "options": [], "stub": True})
    roots = [{"id": nodes[0]["id"], "why": "menu #0: the engine opens it on New Game"},
             {"id": "menu_start_phase_2", "why": "the engine opens it after character creation (D4)"}]
    out = {"format": "uw-flow/1", "generator": "py/flow_export.py", "kit_hash": atlas.get("kit_hash"), "roots": roots,
           "nodes": nodes, "edges": edges}
    with open(a.out, "w", encoding="utf-8") as fh:
        json.dump(out, fh, ensure_ascii=False, indent=1)
    print("nodes %d, options %d, edges %d -> %s" % (
        len(nodes), sum(len(n["options"]) for n in nodes), len(edges), a.out))


if __name__ == "__main__":
    main()
