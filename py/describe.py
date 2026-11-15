#!/usr/bin/env python3
"""Turkish plain-language descriptions of Native's records and operations, written by DeepSeek
from the kit's own source, for graph_viewer and `uw explain`.

For every menu, script, mission template, presentation (and every operation of header_operations)
the model gets the record's source lines (comments included) and answers in JSON:
  {"id", "kisa" (one line: what it is), "uzun" (2-4 sentences: what it does, who starts it),
   "guven" ("yuksek" | "orta" | "dusuk")}
Every name the model writes is checked against the atlas; an unknown name lowers "guven" to
"dusuk" and is listed in "bilinmeyen". Results are cached per table in kit-meta/aciklama/<table>.json
and the run is resumable (existing ids are skipped).

The key is read only from the environment (provider config: .evrim/config.json > delegate);
it is never printed or written.

Usage: python py/describe.py [--tables menus,scripts,...] [--ops] [--limit N] [--workers 8]
"""
import argparse
import concurrent.futures as cf
import json
import os
import re
import sys
import threading
import time
import urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
KIT = os.path.join(ROOT, "Module_system 1.171")
OUT = os.path.join(ROOT, "kit-meta", "aciklama")
ATLAS = os.path.join(ROOT, "target", "uw-staging", "atlas.json")

SYSTEM = (
    "Sen Mount & Blade: Warband Module System (Python 2.7, sürüm 1.171) uzmanısın. Sana kitin kaynak "
    "kodundan kayıtlar verilecek. Her kayıt için Türkçe, teknik olmayan birinin anlayacağı sade bir "
    "açıklama yaz. YALNIZ verilen koda ve yorumlara dayan; emin değilsen 'guven' alanını 'dusuk' yap. "
    "Kodda geçmeyen hiçbir ad uydurma. Oyundaki İngilizce metinleri uzun uzun alıntılama. "
    "Cevap yalnız JSON: {\"items\": [{\"id\": ..., \"kisa\": ..., \"uzun\": ..., \"guven\": ...}]}. "
    "kisa: en çok 90 karakter, ne olduğu (ör. 'Kamp menüsü: dinlenme, eylemler ve hileler'). "
    "uzun: 2-4 cümle: ne yapar, ne zaman/kim tarafından açılır ya da çağrılır, oyuncu ne görür ya da ne değişir. "
    "guven: yuksek | orta | dusuk."
)

KIND_TR = {
    "menus": "oyun menüsü (game_menus: id, bayraklar, metin, mesh, giriş kodu, seçenekler)",
    "scripts": "script (scripts: ad ve işlem bloğu; call_script ile çağrılır)",
    "mission_templates": "görev şablonu (mission_templates: bir sahnede savaş/gezinti kuralları, "
                         "giriş noktaları ve tetikleyiciler)",
    "presentations": "özel ekran (presentations: kendi çizdiği arayüz)",
    "scenes": "sahne (scenes: savaşın ya da gezintinin geçtiği 3B yer)",
    "triggers": "zamanlı tetikleyici (triggers: belli aralıkla koşulu sınar)",
    "simple_triggers": "basit zamanlı tetikleyici (simple_triggers: her N saatte bir çalışır)",
}

lock = threading.Lock()


def provider():
    cfg = json.load(open(os.path.join(ROOT, ".evrim", "config.json"), encoding="utf-8"))
    d = cfg["delegate"]
    p = d["providers"][d["provider"]]
    key = os.environ.get(p["keyEnv"])
    if not key:
        sys.exit("environment variable %s is not set" % p["keyEnv"])
    return p["apiBase"].rstrip("/"), p["model"], key, p.get("price", {})


def ask(api, model, key, user, tries=4):
    body = json.dumps({
        "model": model,
        "messages": [{"role": "system", "content": SYSTEM}, {"role": "user", "content": user}],
        "response_format": {"type": "json_object"},
        "temperature": 0.2,
        "max_tokens": 4000,
    }).encode("utf-8")
    for attempt in range(tries):
        try:
            req = urllib.request.Request(api + "/chat/completions", data=body, headers={
                "Content-Type": "application/json", "Authorization": "Bearer " + key})
            with urllib.request.urlopen(req, timeout=180) as r:
                data = json.loads(r.read().decode("utf-8"))
            text = data["choices"][0]["message"]["content"]
            return json.loads(text), data.get("usage", {})
        except Exception as e:  # network, rate limit, bad JSON: retry with backoff
            err = e
            time.sleep(3 * (attempt + 1))
    raise RuntimeError("request failed: %s" % type(err).__name__)


def source_lines(module):
    path = os.path.join(KIT, module + ".py")
    return open(path, encoding="latin-1").read().splitlines()


def entry_slices(atlas, table_name, max_chars=7000):
    """(id, source text) per entry: from its line to the next entry's line."""
    t = atlas["tables"][table_name]
    lines = source_lines(t["module"])
    entries = [e for e in t["entries"] if e.get("line")]
    starts = sorted(set(e["line"] for e in entries))
    nxt = {a: b for a, b in zip(starts, starts[1:])}
    out = []
    for e in entries:
        ident = e["id"] or "%s%s#%s" % (t.get("id_prefix") or "", (e["name"] or "").lower(), e["dup"])
        a = e["line"] - 1
        b = (nxt.get(e["line"]) or (e["line"] + 200)) - 1
        # include a few comment lines just above the record
        c = a
        while c > 0 and a - c < 6 and lines[c - 1].strip().startswith("#"):
            c -= 1
        text = "\n".join(lines[c:b])
        if len(text) > max_chars:
            text = text[:max_chars] + "\n# ... (kısaltıldı)"
        out.append((ident, "%s.py:%d" % (t["module"], e["line"]), text))
    return out


def op_slices(atlas):
    lines = source_lines("header_operations")
    out = []
    for op in atlas["ops"]:
        if not op.get("line"):
            continue
        a = max(0, op["line"] - 3)
        out.append((op["name"], "header_operations.py:%d" % op["line"], "\n".join(lines[a:op["line"] + 2])))
    return out


NAME_RE = re.compile(r"\b(?:script|mnu|menu|mst|scn|prsnt|trp|itm|pt|fac|qst|spr|snd|str|psys|tableau)_[a-z0-9_]+\b")


def known_names(atlas):
    names = set(op["name"] for op in atlas["ops"])
    for t in atlas["tables"].values():
        for e in t["entries"]:
            if e.get("id"):
                names.add(e["id"])
            if e.get("name"):
                if t.get("ref_tag"):
                    names.add("%s_%s" % (t["ref_tag"], e["name"].lower()))
                if t.get("id_prefix"):
                    names.add("%s%s" % (t["id_prefix"], e["name"].lower()))
    return names


def batches(items, max_chars=9000, max_items=12):
    cur, size = [], 0
    for it in items:
        if cur and (size + len(it[2]) > max_chars or len(cur) >= max_items):
            yield cur
            cur, size = [], 0
        cur.append(it)
        size += len(it[2])
    if cur:
        yield cur


def run_table(name, items, kind_text, api, model, key, known, workers, stats):
    path = os.path.join(OUT, name + ".json")
    done = json.load(open(path, encoding="utf-8")) if os.path.exists(path) else {}
    todo = [it for it in items if it[0] not in done]
    print("%s: %d records, %d to do" % (name, len(items), len(todo)), flush=True)

    def work(batch):
        user = "Kayıt türü: %s\n\n" % kind_text + "\n\n".join(
            "### id: %s  (kaynak %s)\n```python\n%s\n```" % (i, src, text) for i, src, text in batch)
        answer, usage = ask(api, model, key, user)
        got = {}
        by_id = {i: src for i, src, _ in batch}
        for item in answer.get("items", []):
            ident = item.get("id")
            if ident not in by_id:
                continue
            words = (item.get("kisa", "") + " " + item.get("uzun", ""))
            unknown = sorted(set(n for n in NAME_RE.findall(words) if n not in known))
            guven = item.get("guven", "orta")
            if guven not in ("yuksek", "orta", "dusuk"):
                guven = "orta"
            if unknown:
                guven = "dusuk"
            got[ident] = {"kisa": item.get("kisa", "").strip(), "uzun": item.get("uzun", "").strip(),
                          "guven": guven, "kaynak": by_id[ident], "bilinmeyen": unknown}
        return got, usage

    with cf.ThreadPoolExecutor(workers) as pool:
        futs = [pool.submit(work, b) for b in batches(todo)]
        for f in cf.as_completed(futs):
            try:
                got, usage = f.result()
            except Exception as e:
                print("  batch failed: %s" % e, flush=True)
                continue
            with lock:
                done.update(got)
                stats["in"] += usage.get("prompt_tokens", 0)
                stats["cached"] += usage.get("prompt_cache_hit_tokens", 0)
                stats["out"] += usage.get("completion_tokens", 0)
                tmp = path + ".tmp"
                json.dump(dict(sorted(done.items())), open(tmp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
                os.replace(tmp, path)
                print("  %s: %d/%d" % (name, len(done), len(items)), flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--tables", default="menus,mission_templates,presentations,scripts")
    ap.add_argument("--ops", action="store_true")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--workers", type=int, default=8)
    a = ap.parse_args()
    api, model, key, price = provider()
    atlas = json.load(open(ATLAS, encoding="utf-8"))
    known = known_names(atlas)
    os.makedirs(OUT, exist_ok=True)
    stats = {"in": 0, "cached": 0, "out": 0}
    jobs = []
    if a.ops:
        jobs.append(("ops", op_slices(atlas), "Module System işlemi (header_operations: ad = kod, yorumda "
                     "argümanlar). kisa: işlemin ne yaptığı, uzun: argümanlar ve etkisi"))
    for t in [x for x in a.tables.split(",") if x]:
        jobs.append((t, entry_slices(atlas, t), KIND_TR.get(t, t)))
    for name, items, kind_text in jobs:
        if a.limit:
            items = items[:a.limit]
        run_table(name, items, kind_text, api, model, key, known, a.workers, stats)
    cost = ((stats["in"] - stats["cached"]) * price.get("input", 0) + stats["cached"] * price.get("cached", 0)
            + stats["out"] * price.get("output", 0)) / 1e6
    print("tokens in %d (cached %d), out %d, about $%.3f" % (stats["in"], stats["cached"], stats["out"], cost))


if __name__ == "__main__":
    main()
