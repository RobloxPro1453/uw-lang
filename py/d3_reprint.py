#!/usr/bin/env python3
"""D3 (Faz 0): re-print test.

Every table in the atlas JSON is printed back as a Python literal by OUR printer (`py_literal`) and
put in the wrapper: `from native_module_X import *` then `<var> = <printed literal>`. The kit is run
and every output must be byte-identical to a plain kit build. Unlike D2 (an empty patch, which is
tautological), this proves the printer + atlas values carry everything the kit reads.

The kit tells tuples from lists, so the atlas keeps both (tuple = JSON array, list = {"list": [...]}).

Usage: python py/d3_reprint.py --kit "Module_system 1.171" --atlas target/uw-staging/atlas.json
                               --work target/uw-staging [--tables scripts,items]
"""
import argparse
import json
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import kitrun  # noqa: E402


def py_literal(v):
    """Python 2.7 source for a JSON value. Strings become byte literals whose bytes are the
    latin-1 encoding of the text (atlas_dump decoded them as latin-1)."""
    if v is None:
        return "None"
    if v is True:
        return "True"
    if v is False:
        return "False"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        return repr(v)  # shortest round-trip repr; Python 2.7 parses it to the same double
    if isinstance(v, str):
        out = []
        for b in v.encode("latin-1"):
            c = chr(b)
            if c == "\\" or c == '"':
                out.append("\\" + c)
            elif 32 <= b < 127:
                out.append(c)
            else:
                out.append("\\x%02x" % b)
        return '"' + "".join(out) + '"'
    if isinstance(v, list):  # a Python tuple (see atlas_dump.jsonable)
        return "(" + "".join(py_literal(x) + "," for x in v) + ")"
    if isinstance(v, dict) and list(v) == ["list"]:
        return "[" + ",".join(py_literal(x) for x in v["list"]) + "]"
    raise TypeError(type(v))


def build_overlay(kit, atlas, overlay, only):
    if os.path.exists(overlay):
        shutil.rmtree(overlay)
    os.makedirs(overlay)
    by_module = {}
    for table, t in atlas["tables"].items():
        if only and table not in only:
            continue
        by_module.setdefault(t["module"], []).append(t)
    for module, tables in sorted(by_module.items()):
        shutil.copy2(os.path.join(kit, module + ".py"), os.path.join(overlay, "native_" + module + ".py"))
        with open(os.path.join(overlay, module + ".py"), "w", encoding="ascii") as fh:
            fh.write("# uw: D3 re-print\nfrom native_%s import *\n" % module)
            for t in tables:
                vals = [e["value"] for e in sorted(t["entries"], key=lambda e: e["index"])]
                fh.write("%s = [\n" % t["var"])
                for v in vals:
                    fh.write(" " + py_literal(v) + ",\n")
                fh.write("]\n")
    return sorted(by_module)


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--kit", required=True)
    ap.add_argument("--atlas", required=True)
    ap.add_argument("--work", required=True)
    ap.add_argument("--tables")
    ap.add_argument("--python", default=kitrun.DEFAULT_PY27)
    a = ap.parse_args(argv)
    kit, work = os.path.abspath(a.kit), os.path.abspath(a.work)
    atlas = json.load(open(a.atlas, encoding="utf-8"))
    only = set(a.tables.split(",")) if a.tables else None

    base_dir = os.path.join(work, "d1")
    if not os.path.exists(os.path.join(base_dir, "report.json")):
        kitrun.main(["--kit", kit, "--staging", base_dir, "--python", a.python])
    overlay = os.path.join(work, "d3-overlay")
    patch_dir = os.path.join(work, "d3-patch")
    modules = build_overlay(kit, atlas, overlay, only)
    rc = kitrun.main(["--kit", kit, "--staging", patch_dir, "--python", a.python, "--overlay", overlay])
    base = json.load(open(os.path.join(base_dir, "report.json"), encoding="utf-8"))
    patch = json.load(open(os.path.join(patch_dir, "report.json"), encoding="utf-8"))
    diffs = sorted(k for k in set(base["outputs"]) | set(patch["outputs"])
                   if base["outputs"].get(k) != patch["outputs"].get(k))
    print("D3: %d modules re-printed, %d outputs, %d differ, failed steps: %s" % (
        len(modules), len(base["outputs"]), len(diffs), patch["failed_steps"]))
    for d in diffs:
        print("  differs:", d)
    for s in patch["steps"]:
        if s["exit"]:
            print("  %s: %s" % (s["step"], (s["stderr"].strip().splitlines() or ["?"])[-1]))
    return 0 if (rc == 0 and not diffs) else 1


if __name__ == "__main__":
    sys.exit(main())
