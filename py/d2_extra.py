#!/usr/bin/env python3
"""D2 remaining sub-tests: (a) standalone import, (d) two builds, (e) Native hash.

(a) In an empty-patch staging copy of the kit (py/d2_empty_patch.py overlay), every wrapped
    `module_X.py` must import on its own in a fresh Python 2.7 process (exit 0).
(d) The empty patch built twice into two staging folders must give identical output files.
(e) `kitrun.kit_hash(kit)` must equal the baseline in kit-meta/native.baseline.json.

Usage: python py/d2_extra.py --kit "Module_system 1.171" --work target/uw-staging
Exit 0 only when all three checks pass; details in <work>/d2-extra-result.json.
"""
import argparse
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import kitrun  # noqa: E402
import d2_empty_patch  # noqa: E402

BASELINE = os.path.join(os.path.dirname(HERE), "kit-meta", "native.baseline.json")


def py27(staging, code, python):
    """Run `code` with Python 2.7 in the staging copy; returns (exit, stdout, stderr)."""
    env = {k: v for k, v in os.environ.items() if not k.upper().startswith("PYTHON")}
    p = subprocess.run([python, "-B", "-E", "-s", "-c", code], cwd=staging, env=env,
                       capture_output=True)
    return (p.returncode,
            p.stdout.decode("latin-1").replace("\r\n", "\n"),
            p.stderr.decode("latin-1").replace("\r\n", "\n"))


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--kit", required=True)
    ap.add_argument("--work", required=True)
    ap.add_argument("--python", default=kitrun.DEFAULT_PY27)
    a = ap.parse_args(argv)
    kit, work = os.path.abspath(a.kit), os.path.abspath(a.work)
    if not os.path.isfile(os.path.join(kit, "build_module.bat")):
        sys.exit("%s is not a Module System kit (no build_module.bat)" % kit)

    overlay = os.path.join(work, "d2x-overlay")
    wrapped = d2_empty_patch.build_overlay(kit, overlay)

    # (a) every wrapper imports on its own, one fresh Python 2.7 process per module.
    import_dir = os.path.join(work, "d2x-import")
    kitrun.stage(kit, import_dir, False, overlay)
    imports = {}
    for mod in wrapped:
        rc, _out, err = py27(import_dir, "import %s\n" % mod, a.python)
        imports[mod] = {"exit": rc, "stderr": err.strip()[-1000:]}
    failed_imports = sorted(m for m in imports if imports[m]["exit"] != 0)

    # (d) the same empty patch built twice: identical output files.
    build_dirs = [os.path.join(work, "d2x-build1"), os.path.join(work, "d2x-build2")]
    build_exits = [kitrun.main(["--kit", kit, "--staging", d, "--python", a.python,
                                "--overlay", overlay]) for d in build_dirs]
    reports = [json.load(open(os.path.join(d, "report.json"), encoding="utf-8")) for d in build_dirs]
    differing = sorted(k for k in set(reports[0]["outputs"]) | set(reports[1]["outputs"])
                       if reports[0]["outputs"].get(k) != reports[1]["outputs"].get(k))

    # (e) the kit is the expected Native.
    baseline = json.load(open(BASELINE, encoding="utf-8"))
    kit_hash = kitrun.kit_hash(kit)

    checks = {
        "a_each_wrapper_imports_alone": bool(wrapped) and not failed_imports,
        "d_two_builds_identical": (build_exits == [0, 0] and not differing
                                   and bool(reports[0]["outputs"])),
        "e_kit_is_expected_native": kit_hash == baseline["kit_hash"],
    }
    result = {
        "checks": checks,
        "wrapped_modules": wrapped,
        "imports": imports,
        "failed_imports": failed_imports,
        "build_exits": build_exits,
        "differing_outputs": differing,
        "outputs": len(reports[0]["outputs"]),
        "kit_hash": kit_hash,
        "expected_kit_hash": baseline["kit_hash"],
    }
    with open(os.path.join(work, "d2-extra-result.json"), "w", encoding="utf-8") as fh:
        json.dump(result, fh, indent=1)
    for name, good in sorted(checks.items()):
        print("  %-32s %s" % (name, "ok" if good else "FAIL"))
    for m in failed_imports:
        print("  import %s failed: %s" % (m, imports[m]["stderr"]))
    for name in differing:
        print("  differs:", name)
    if not checks["e_kit_is_expected_native"]:
        print("  kit hash %s, expected %s" % (kit_hash, baseline["kit_hash"]))
    return 0 if all(checks.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
