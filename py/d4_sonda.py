#!/usr/bin/env python3
"""D4 (Faz 0, in-game probe): how does a new game start, and can UW cut the start chain?

Three variants, each a hand-written wrapper over Native built with the real kit through
py/uw_runtime.py, installed as its own test module next to Native (never over Native or the
owner's modules):

  uw_sonda_a  menu 0 (start_game_0) on-enter body = change_screen_return
  uw_sonda_b  menu 0 (start_game_0) on-enter body = change_screen_map
  uw_sonda_c  menu 0 untouched (character creation as in Native);
              menu 1 (start_phase_2, the prologue menu nothing in Native jumps to) on-enter body =
              display_message + change_screen_map

All three append to script game_start: party_relocate_near_party(p_main_party, p_town_6, 3)
(p_town_6 = Praven, module_parties.py:82) and a display_message, so the owner can see whether
game_start ran and whether the party starts next to Praven. Every name used here was looked up
with `uw explain` in the kit (header_operations.py / module_*.py), none typed from memory.

Usage:
  python py/d4_sonda.py --kit "Module_system 1.171" --work target/uw-staging \
      [--modules "C:/Program Files (x86)/Steam/steamapps/common/MountBlade Warband/Modules"]

Without --modules it only builds (target/uw-staging/d4-<v>/out). With --modules it copies
<modules>/Native to <modules>/uw_sonda_<v> and puts the built .txt files over the copy. A
uw_sonda_<v> folder is replaced only when it carries the .uw-sonda marker this script writes.
"""
import argparse
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import kitrun  # noqa: E402
import d2_empty_patch  # noqa: E402

MARKER = ".uw-sonda"

HEAD = '''# uw: D4 probe, variant %(v)s (hand-written wrapper)
from native_module_%(mod)s import *
from header_common import *
from header_operations import *
import uw_runtime as uw
'''

SCRIPTS = HEAD + '''scripts = uw.append_ops(scripts, "game_start", 1, [
    (party_relocate_near_party, "p_main_party", "p_town_6", 3),
    (display_message, "@UW sonda %(v)s: game_start ran, party moved next to Praven"),
])
'''

MENUS = {
    "a": HEAD + 'game_menus = uw.replace_body(game_menus, "start_game_0", 4, [change_screen_return])\n',
    "b": HEAD + 'game_menus = uw.replace_body(game_menus, "start_game_0", 4, [change_screen_map])\n',
    "c": HEAD + '''game_menus = uw.replace_body(game_menus, "start_phase_2", 4, [
    (display_message, "@UW sonda c: menu 1 (start_phase_2) entered"),
    change_screen_map,
])
''',
}


def build(kit, work, v, python):
    overlay = os.path.join(work, "d4-%s-overlay" % v)
    d2_empty_patch.build_overlay(kit, overlay)
    shutil.copy2(os.path.join(HERE, "uw_runtime.py"), os.path.join(overlay, "uw_runtime.py"))
    with open(os.path.join(overlay, "module_scripts.py"), "w", encoding="ascii") as fh:
        fh.write(SCRIPTS % {"v": v, "mod": "scripts"})
    with open(os.path.join(overlay, "module_game_menus.py"), "w", encoding="ascii") as fh:
        fh.write(MENUS[v] % {"v": v, "mod": "game_menus"})
    staging = os.path.join(work, "d4-%s" % v)
    rc = kitrun.main(["--kit", kit, "--staging", staging, "--python", python, "--overlay", overlay])
    if rc != 0:
        sys.exit("variant %s: kit build failed, see %s/report.json" % (v, staging))
    return os.path.join(staging, "out")


def install(out_dir, modules, v):
    native = os.path.join(modules, "Native")
    dest = os.path.join(modules, "uw_sonda_%s" % v)
    if not os.path.isfile(os.path.join(native, "module.ini")):
        sys.exit("%s is not a Warband module folder" % native)
    if os.path.exists(dest):
        if not os.path.isfile(os.path.join(dest, MARKER)):
            sys.exit("%s exists and was not made by this script; refusing to touch it" % dest)
        shutil.rmtree(dest)
    shutil.copytree(native, dest)
    with open(os.path.join(dest, MARKER), "w") as fh:
        fh.write("made by universal_warband py/d4_sonda.py; safe to delete\n")
    n = 0
    for root, _dirs, files in os.walk(out_dir):
        for f in files:
            src = os.path.join(root, f)
            dst = os.path.join(dest, os.path.relpath(src, out_dir))
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy2(src, dst)
            n += 1
    print("installed %s (%d built files over a Native copy)" % (dest, n))


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--kit", required=True)
    ap.add_argument("--work", required=True)
    ap.add_argument("--modules")
    ap.add_argument("--python", default=kitrun.DEFAULT_PY27)
    ap.add_argument("--variants", default="abc")
    a = ap.parse_args(argv)
    kit, work = os.path.abspath(a.kit), os.path.abspath(a.work)
    for v in a.variants:
        out = build(kit, work, v, a.python)
        if a.modules:
            install(out, os.path.abspath(a.modules), v)
    return 0


if __name__ == "__main__":
    sys.exit(main())
