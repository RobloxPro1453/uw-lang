#!/usr/bin/env python3
"""D2 (Faz 0): empty-patch round trip.

Every kit `module_X.py` (except module_info.py, which staging writes) is put into the staging copy as
`native_module_X.py`, unchanged, and replaced by a thin wrapper `module_X.py` that only does
`from native_module_X import *`. The kit is then run (py/kitrun.py) and every output file must be
byte-identical to a plain kit build (D1).

Usage: python py/d2_empty_patch.py --kit "Module_system 1.171" --work target/uw-staging
Exit 0 only when all outputs match and every native_* file has the kit file's hash.
"""
import argparse
import hashlib
import json
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import kitrun  # noqa: E402

WRAPPER = "# uw: empty patch (D2)\nfrom native_%s import *\n"


def build_overlay(kit, overlay):
    if os.path.exists(overlay):
        shutil.rmtree(overlay)
    os.makedirs(overlay)
    wrapped = []
    for name in sorted(os.listdir(kit)):
        if not (name.startswith("module_") and name.endswith(".py")) or name == "module_info.py":
            continue
        mod = name[:-3]
        shutil.copy2(os.path.join(kit, name), os.path.join(overlay, "native_" + name))
        with open(os.path.join(overlay, name), "w", encoding="ascii") as fh:
            fh.write(WRAPPER % mod)
        wrapped.append(mod)
    return wrapped


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--kit", required=True)
    ap.add_argument("--work", required=True)
    ap.add_argument("--python", default=kitrun.DEFAULT_PY27)
    a = ap.parse_args(argv)
    kit, work = os.path.abspath(a.kit), os.path.abspath(a.work)

    base_dir = os.path.join(work, "d2-base")
    patch_dir = os.path.join(work, "d2-patch")
    overlay = os.path.join(work, "d2-overlay")
    wrapped = build_overlay(kit, overlay)

    rc = kitrun.main(["--kit", kit, "--staging", base_dir, "--python", a.python])
    rc |= kitrun.main(["--kit", kit, "--staging", patch_dir, "--python", a.python, "--overlay", overlay])
    base = json.load(open(os.path.join(base_dir, "report.json"), encoding="utf-8"))
    patch = json.load(open(os.path.join(patch_dir, "report.json"), encoding="utf-8"))

    diffs = sorted(k for k in set(base["outputs"]) | set(patch["outputs"])
                   if base["outputs"].get(k) != patch["outputs"].get(k))
    native_hash_ok = all(
        kitrun.sha256_file(os.path.join(kit, m + ".py"))
        == kitrun.sha256_file(os.path.join(patch_dir, "native_" + m + ".py")) for m in wrapped)
    result = {"wrapped_modules": wrapped, "outputs": len(base["outputs"]), "differing_outputs": diffs,
              "failed_steps": patch["failed_steps"], "native_hash_ok": native_hash_ok,
              "kit_hash": base["kit_hash"]}
    with open(os.path.join(work, "d2-result.json"), "w", encoding="utf-8") as fh:
        json.dump(result, fh, indent=1)
    print("D2: %d wrapped modules, %d outputs, %d differ, native hash ok: %s" % (
        len(wrapped), len(base["outputs"]), len(diffs), native_hash_ok))
    for d in diffs:
        print("  differs:", d)
    return 0 if (rc == 0 and not diffs and native_hash_ok and base["outputs"]) else 1


if __name__ == "__main__":
    sys.exit(main())
