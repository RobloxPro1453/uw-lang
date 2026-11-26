# -*- coding: utf-8 -*-
"""Writes kit-meta/hooks.toml: every engine hook (script named game_*) with how its result goes
back to the engine. Python 2.7 ONLY, run with -B -E -s inside a kit staging copy:

    C:/Python27/python.exe -B -E -s <repo>/py/hooks_meta.py <out.toml>

Two kinds of evidence, kept apart because they disagree in places:
  comment  the kit's own comment block right above the script (INPUT/OUTPUT lines), verbatim
  body     what the script body does, read from the values: set_trigger_result calls, writes to
           reg0 (header_common.reg0) and to s0 by string operations (operations whose header
           signature names a <string_register> first operand)
`channels` lists every way the kit shows a result going back: trigger_result / result_string from
the body (set_trigger_result / set_result_string calls) or the comment, s0 / reg0 from the comment
only (bodies use s0 and reg0 as scratch too). `review` says where comment and body disagree.
It is a reading of the kit, not an engine experiment.
"""
import io
import re
import sys


def comment_block(lines, line_no):
    """Comment lines directly above 1-based `line_no`, '#' stripped, blank lines skipped."""
    out = []
    i = line_no - 2
    while i >= 0:
        s = lines[i].strip()
        if s.startswith("#"):
            out.append(s.lstrip("#").strip())
        elif s:
            break
        i -= 1
    out.reverse()
    return [l for l in out if l]


def walk(block, fn):
    for st in block:
        if isinstance(st, tuple) and st and isinstance(st[0], (int, long)):
            fn(st)
        if isinstance(st, (list, tuple)):
            for x in st:
                if isinstance(x, list):
                    walk(x, fn)


def toml_str(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def main():
    import os
    sys.path.insert(0, os.getcwd())
    import header_common as hc
    import header_operations as ho
    from module_scripts import scripts
    out_path = sys.argv[1]

    with io.open("header_operations.py", "r", encoding="latin-1") as fh:
        hlines = fh.read().splitlines()
    names, str_reg_ops = {}, set()
    for line in hlines:
        m = re.match(r"^([A-Za-z_]\w*)\s*=\s*(0x[0-9a-fA-F]+|\d+)\b(.*)$", line)
        if not m:
            continue
        code = getattr(ho, m.group(1))
        names.setdefault(code, m.group(1))
        sig = m.group(3)
        first = re.search(r"\(\s*%s\s*,\s*<([^>]*)>" % re.escape(m.group(1)), sig)
        if first and "string_register" in first.group(1):
            str_reg_ops.add(code)
    flags = ho.neg | ho.this_or_next

    with io.open("module_scripts.py", "r", encoding="latin-1") as fh:
        src = fh.read().splitlines()
    starts = {}
    pos = 0
    text = "\n".join(src)
    for s in scripts:
        m = re.compile(r'[\(\[]\s*"%s"\s*,' % re.escape(s[0])).search(text, pos)
        if m:
            starts[s[0]] = text.count("\n", 0, text.index('"', m.start())) + 1
            pos = m.end()

    rows = []
    for s in scripts:
        if not s[0].startswith("game_"):
            continue
        ev = {"set_trigger_result": 0, "set_result_string": 0, "reg0": 0, "s0": 0}

        def look(st):
            code = st[0] & ~flags
            if code == ho.set_trigger_result:
                ev["set_trigger_result"] += 1
            elif code == ho.set_result_string:
                ev["set_result_string"] += 1
            elif len(st) > 1 and code in ho.lhs_operations and st[1] == hc.reg0:
                ev["reg0"] += 1
            elif len(st) > 1 and code in str_reg_ops and st[1] == hc.s0:
                ev["s0"] += 1
        walk(s[1], look)
        line = starts.get(s[0])
        comment = comment_block(src, line) if line else []
        out_text = " ".join(c for c in comment if re.match(r"(?i)output", c)).strip()
        says = lambda pat: re.search(pat, out_text, re.I) is not None
        channels, review = [], []
        if ev["set_trigger_result"] or says(r"trigger_result"):
            channels.append("trigger_result")
        if ev["set_result_string"] or says(r"result[ _]string"):
            channels.append("result_string")
        if says(r"(?<![a-z0-9_])s0(?![0-9])"):
            channels.append("s0")
        if says(r"(?<![a-z0-9_])reg0(?![0-9])"):
            if ev["set_trigger_result"]:
                review.append("comment says reg0, body calls set_trigger_result")
            else:
                channels.append("reg0")
        silent = not re.sub(r"(?i)output\s*:?|none", "", out_text).strip(" .:,")
        if not channels and (ev["reg0"] or ev["s0"]):
            if not out_text:
                review.append("no OUTPUT comment; body writes reg0/s0 (scratch or result?)")
            elif not silent:
                review.append("OUTPUT comment names no channel; body writes reg0/s0")
        rows.append((s[0], line, comment, ev, channels, review))

    w = []
    w.append("# Engine hooks: scripts named game_* (the engine calls them itself) and their return")
    w.append("# channels. GENERATED by py/hooks_meta.py from the kit; see its docstring for the rules.")
    w.append("# channels: trigger_result | result_string | s0 | reg0; [] = nothing returned that the kit shows.")
    w.append("# Reading of the kit only, not an engine experiment. `review` = comment and body disagree.")
    w.append("")
    for name, line, comment, ev, channels, review in rows:
        w.append("[%s]" % name)
        w.append("source = %s" % toml_str("module_scripts.py:%s" % line))
        w.append("channels = [%s]" % ", ".join(toml_str(c) for c in channels))
        w.append("body = { set_trigger_result = %d, set_result_string = %d, reg0_writes = %d, "
                 "s0_writes = %d }" % (ev["set_trigger_result"], ev["set_result_string"],
                                       ev["reg0"], ev["s0"]))
        if review:
            w.append("review = [%s]" % ", ".join(toml_str(r) for r in review))
        w.append("comment = [%s]" % ", ".join(toml_str(c) for c in comment))
        w.append("")
    with io.open(out_path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write(u"\n".join(w))
    print("hooks: %d" % len(rows))


if __name__ == "__main__":
    main()
