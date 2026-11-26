#!/usr/bin/env python3
"""D2 sub-tests (b) shared object and (c) `name#n`, through py/uw_runtime.py, on the real kit.

(b) Native puts one trigger object into many mission templates. The patch prepends one harmless
    operation, `(assign, reg0, reg0)`, to that trigger's consequences in ONE template only.
    Expected: only mission_templates.txt differs from a plain build, only that template's block
    differs inside it, and in Python every other template still holds the original object.
(c) `tutorial_spear` is defined twice in module_items.py. The patch renames `tutorial_spear#1`.
    Expected: only the first occurrence changes; the second is the very same object; a bare
    `tutorial_spear` is refused.

Nothing is typed in from memory: the shared trigger, its templates and the duplicated item are
found by a probe run in a staging copy of the kit (Python 2.7, -B -E -s).

Usage: python py/d2_runtime.py --kit "Module_system 1.171" --work target/uw-staging
Exit 0 only when every expectation holds; details in <work>/d2-runtime-result.json.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import kitrun  # noqa: E402
import d2_empty_patch  # noqa: E402

PROBE = r'''
import json
from process_common import convert_to_identifier
from module_mission_templates import mission_templates
from module_items import items
import uw_runtime as uw

def key(n):
    return convert_to_identifier(n).lower()

def ref(lst, i):
    k = key(lst[i][0])
    same = [j for j in range(len(lst)) if key(lst[j][0]) == k]
    return lst[i][0] if len(same) == 1 else "%s#%d" % (lst[i][0], same.index(i) + 1)

users = {}
for t, tpl in enumerate(mission_templates):
    for k, trig in enumerate(tpl[5]):
        users.setdefault(id(trig), (trig, []))[1].append((t, k))
trig, places = max(users.values(), key=lambda v: len(set(t for t, _ in v[1])))
t, k = places[0]
dup = None
for name in sorted(set(key(e[0]) for e in items)):
    hits = [i for i, e in enumerate(items) if key(e[0]) == name]
    if len(hits) > 1:
        dup = (items[hits[0]][0], hits, [items[h][1] for h in hits])
        break
print(json.dumps({
    "template": ref(mission_templates, t), "template_index": t, "trigger": k,
    "consequences_is_list": isinstance(trig[4], list), "consequences_len": len(trig[4]),
    "users": sorted(set(x for x, _ in places)),
    "item": dup[0], "item_hits": dup[1], "item_names": dup[2],
}))
'''

CHECK = r'''
import json, sys
import uw_runtime as uw
import module_mission_templates as w_mt, native_module_mission_templates as n_mt
import module_items as w_it, native_module_items as n_it
p = json.loads(sys.argv[1])
t, k = p["template_index"], p["trigger"]
shared = n_mt.mission_templates[t][5][k]
ok = {}
ok["target_trigger_is_new_object"] = w_mt.mission_templates[t][5][k] is not shared
ok["target_got_one_op_in_front"] = (
    len(w_mt.mission_templates[t][5][k][4]) == len(shared[4]) + 1
    and w_mt.mission_templates[t][5][k][4][1:] == shared[4])
ok["native_shared_unchanged"] = len(shared[4]) == p["consequences_len"]
ok["other_users_keep_same_object"] = all(
    w_mt.mission_templates[u] is n_mt.mission_templates[u] for u in p["users"] if u != t)
ok["other_templates_same_object"] = all(
    w_mt.mission_templates[i] is n_mt.mission_templates[i]
    for i in range(len(n_mt.mission_templates)) if i != t)
first, second = p["item_hits"][0], p["item_hits"][1]
ok["item_first_renamed"] = w_it.items[first][1] == "UW Probe Item"
ok["item_second_same_object"] = w_it.items[second] is n_it.items[second]
ok["item_native_unchanged"] = n_it.items[first][1] == p["item_names"][0]
try:
    uw.find(n_it.items, p["item"])
    ok["bare_duplicate_refused"] = False
except uw.UwRuntimeError:
    ok["bare_duplicate_refused"] = True
print(json.dumps(ok))
'''

WRAP_MT = '''# uw: D2 sub-test (b), shared trigger object
from native_module_mission_templates import *
from header_common import *
from header_operations import *
import uw_runtime as uw
mission_templates = uw.prepend_ops(mission_templates, %r, (5, %d, 4), [(assign, reg0, reg0)])
'''

WRAP_IT = '''# uw: D2 sub-test (c), name#n
from native_module_items import *
import uw_runtime as uw
items = uw.replace_field(items, %r, 1, "UW Probe Item")
'''


def py27(staging, code, python, *args):
    env = {k: v for k, v in os.environ.items() if not k.upper().startswith("PYTHON")}
    p = subprocess.run([python, "-B", "-E", "-s", "-c", code] + list(args), cwd=staging, env=env,
                       capture_output=True)
    if p.returncode != 0:
        sys.exit("python 2.7 failed in %s:\n%s" % (staging, p.stderr.decode("latin-1")))
    return json.loads(p.stdout.decode("latin-1").strip().splitlines()[-1])


def blocks(path, start):
    """Text split into blocks, each starting at a line that begins with `start`."""
    out, cur = [], []
    with open(path, encoding="latin-1") as fh:
        for line in fh:
            if line.startswith(start) and cur:
                out.append(cur)
                cur = []
            cur.append(line)
    out.append(cur)
    return out


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--kit", required=True)
    ap.add_argument("--work", required=True)
    ap.add_argument("--python", default=kitrun.DEFAULT_PY27)
    a = ap.parse_args(argv)
    kit, work = os.path.abspath(a.kit), os.path.abspath(a.work)

    # Probe in an empty-patch staging copy with uw_runtime present.
    overlay = os.path.join(work, "d2rt-overlay")
    d2_empty_patch.build_overlay(kit, overlay)
    shutil.copy2(os.path.join(HERE, "uw_runtime.py"), os.path.join(overlay, "uw_runtime.py"))
    probe_dir = os.path.join(work, "d2rt-probe")
    kitrun.stage(kit, probe_dir, False, overlay)
    p = py27(probe_dir, PROBE, a.python)
    print("probe: template %s (#%d) trigger %d shared by %d templates; item %s at %s" % (
        p["template"], p["template_index"], p["trigger"], len(p["users"]), p["item"], p["item_hits"]))
    if not p["consequences_is_list"] or len(p["users"]) < 2:
        sys.exit("probe: no shared trigger with a list body found")

    with open(os.path.join(overlay, "module_mission_templates.py"), "w", encoding="ascii") as fh:
        fh.write(WRAP_MT % (p["template"], p["trigger"]))
    with open(os.path.join(overlay, "module_items.py"), "w", encoding="ascii") as fh:
        fh.write(WRAP_IT % ("%s#1" % p["item"]))

    base_dir = os.path.join(work, "d2rt-base")
    patch_dir = os.path.join(work, "d2rt-patch")
    rc = kitrun.main(["--kit", kit, "--staging", base_dir, "--python", a.python])
    rc |= kitrun.main(["--kit", kit, "--staging", patch_dir, "--python", a.python,
                       "--overlay", overlay])
    base = json.load(open(os.path.join(base_dir, "report.json"), encoding="utf-8"))
    patch = json.load(open(os.path.join(patch_dir, "report.json"), encoding="utf-8"))
    diffs = sorted(k for k in set(base["outputs"]) | set(patch["outputs"])
                   if base["outputs"].get(k) != patch["outputs"].get(k))

    ok = py27(patch_dir, CHECK, a.python, json.dumps(p))
    ok["kit_steps_clean"] = rc == 0 and not patch["failed_steps"]
    ok["only_expected_outputs_differ"] = diffs == ["item_kinds1.txt", "mission_templates.txt"]

    mt_b = blocks(os.path.join(base_dir, "out", "mission_templates.txt"), "mst_")
    mt_p = blocks(os.path.join(patch_dir, "out", "mission_templates.txt"), "mst_")
    changed_mt = [i for i, (x, y) in enumerate(zip(mt_b, mt_p)) if x != y]
    # Block 0 is the file header, so template n is block n + 1.
    ok["only_target_template_block_differs"] = (
        len(mt_b) == len(mt_p) and changed_mt == [p["template_index"] + 1])

    it_b = blocks(os.path.join(base_dir, "out", "item_kinds1.txt"), " itm_")
    it_p = blocks(os.path.join(patch_dir, "out", "item_kinds1.txt"), " itm_")
    changed_it = [i for i, (x, y) in enumerate(zip(it_b, it_p)) if x != y]
    ok["only_first_item_block_differs"] = (
        len(it_b) == len(it_p) and changed_it == [p["item_hits"][0] + 1]
        and any("UW_Probe_Item" in line for line in it_p[p["item_hits"][0] + 1]))

    result = {"probe": p, "differing_outputs": diffs, "changed_template_blocks": changed_mt,
              "changed_item_blocks": changed_it, "checks": ok, "kit_hash": base["kit_hash"]}
    with open(os.path.join(work, "d2-runtime-result.json"), "w", encoding="utf-8") as fh:
        json.dump(result, fh, indent=1)
    for name, good in sorted(ok.items()):
        print("  %-36s %s" % (name, "ok" if good else "FAIL"))
    print("differing outputs:", diffs)
    return 0 if all(ok.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
