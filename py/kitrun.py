#!/usr/bin/env python3
"""Faz 0 prototype of the kit runner (later ported to the `uw_kitrun` crate).

Copies the Module System kit into a fresh staging folder (the kit itself is never written), points
`export_dir` at `<staging>/out/`, then runs the kit's `process_*` steps one by one with Python 2.7
(`-B -E -s`), in the order `build_module.bat` lists them. The bat itself is never called.

Writes `<staging>/report.json`: per step exit code, seconds, stdout/stderr, error/warning lines;
the output files with sha256; the kit hash.

Usage:
  python py/kitrun.py --kit "Module_system 1.171" --staging target/uw-staging/d1 [--no-ids]
                      [--python C:/Python27/python.exe] [--overlay DIR]

--no-ids    leave the kit's ID_*.py files out of the staging copy (D1 bootstrap check).
--overlay   copy every file of DIR over the staging copy after the kit (D2: wrappers + native_*).
"""
import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time

DEFAULT_PY27 = r"C:\Python27\python.exe"
STEP_RE = re.compile(r"python\.exe\s+(process_\w+\.py)", re.IGNORECASE)
# Lines the kit prints when something is wrong. Kept broad on purpose: the baseline records them all.
PROBLEM_RE = re.compile(r"error|warning|exception|traceback|unable|not found|invalid", re.IGNORECASE)


def kit_files(kit):
    """Relative paths of the kit's files, without bytecode caches. Sorted, '/' separated."""
    out = []
    for root, dirs, files in os.walk(kit):
        dirs[:] = sorted(d for d in dirs if d != "__pycache__")
        for f in sorted(files):
            if f.endswith(".pyc"):
                continue
            out.append(os.path.relpath(os.path.join(root, f), kit).replace(os.sep, "/"))
    return out


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def kit_hash(kit):
    """Hash of every kit file except module_info.py (it only holds the user's export path)."""
    h = hashlib.sha256()
    for rel in kit_files(kit):
        if rel == "module_info.py":
            continue
        h.update(rel.encode("utf-8") + b"\0" + sha256_file(os.path.join(kit, rel)).encode() + b"\n")
    return h.hexdigest()


def steps_from_bat(kit):
    with open(os.path.join(kit, "build_module.bat"), "r", encoding="latin-1") as fh:
        return STEP_RE.findall(fh.read())


def stage(kit, staging, no_ids, overlay):
    if os.path.exists(staging):
        shutil.rmtree(staging)
    os.makedirs(staging)
    for rel in kit_files(kit):
        name = os.path.basename(rel)
        if no_ids and name.startswith("ID_") and name.endswith(".py"):
            continue
        dst = os.path.join(staging, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(os.path.join(kit, rel), dst)
    if overlay:
        for root, _dirs, files in os.walk(overlay):
            for f in files:
                src = os.path.join(root, f)
                dst = os.path.join(staging, os.path.relpath(src, overlay))
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copy2(src, dst)
    out_dir = os.path.join(staging, "out")
    os.makedirs(out_dir)
    export = os.path.abspath(out_dir).replace("\\", "/") + "/"
    with open(os.path.join(staging, "module_info.py"), "w", encoding="ascii") as fh:
        fh.write('# written by uw kitrun\nexport_dir = "%s"\n' % export)
    return out_dir


def run_steps(staging, steps, python):
    env = {k: v for k, v in os.environ.items() if not k.upper().startswith("PYTHON")}
    results = []
    for step in steps:
        t0 = time.time()
        p = subprocess.run([python, "-B", "-E", "-s", step], cwd=staging, env=env,
                           capture_output=True)
        dt = round(time.time() - t0, 3)
        out = p.stdout.decode("latin-1").replace("\r\n", "\n")
        err = p.stderr.decode("latin-1").replace("\r\n", "\n")
        problems = [l for l in (out + "\n" + err).splitlines() if PROBLEM_RE.search(l)]
        results.append({"step": step, "exit": p.returncode, "seconds": dt,
                        "stdout": out, "stderr": err, "problems": problems})
    return results


def outputs(out_dir):
    res = {}
    for root, _dirs, files in os.walk(out_dir):
        for f in sorted(files):
            path = os.path.join(root, f)
            res[os.path.relpath(path, out_dir).replace(os.sep, "/")] = {
                "bytes": os.path.getsize(path), "sha256": sha256_file(path)}
    return dict(sorted(res.items()))


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--kit", required=True)
    ap.add_argument("--staging", required=True)
    ap.add_argument("--python", default=DEFAULT_PY27)
    ap.add_argument("--no-ids", action="store_true")
    ap.add_argument("--overlay")
    a = ap.parse_args(argv)

    kit = os.path.abspath(a.kit)
    staging = os.path.abspath(a.staging)
    if os.path.commonpath([kit, staging]) == kit:
        sys.exit("staging must not be inside the kit")
    steps = steps_from_bat(kit)
    t0 = time.time()
    out_dir = stage(kit, staging, a.no_ids, a.overlay)
    results = run_steps(staging, steps, a.python)
    report = {
        "kit_hash": kit_hash(kit),
        "no_ids": a.no_ids,
        "overlay": a.overlay,
        "steps": results,
        "failed_steps": [r["step"] for r in results if r["exit"] != 0],
        "total_seconds": round(time.time() - t0, 3),
        "outputs": outputs(out_dir),
    }
    with open(os.path.join(staging, "report.json"), "w", encoding="utf-8") as fh:
        json.dump(report, fh, indent=1, ensure_ascii=False)
    print("steps: %d, failed: %d, outputs: %d, seconds: %.1f" % (
        len(results), len(report["failed_steps"]), len(report["outputs"]), report["total_seconds"]))
    for r in results:
        if r["exit"] != 0 or r["problems"]:
            print("  %s exit=%d problems=%d" % (r["step"], r["exit"], len(r["problems"])))
    return 1 if report["failed_steps"] else 0


if __name__ == "__main__":
    sys.exit(main())
