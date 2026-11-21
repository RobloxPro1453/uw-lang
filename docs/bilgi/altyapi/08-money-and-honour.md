# Altyapı taraması — 08 Money and Honour

Tek satır: ChroniclesStudio'nun Otuz Yıl Savaşları temalı, Çince yazılmış bir total conversion'ı; kendi
kaynağı MIT (`README.md:3-9`). Module System 1.171 kitinin üstüne `module_scripts.py` +2320 satır
(53103 / 50783), `module_presentations.py` +2021 (15643 / 13622), `module_mission_templates.py` +1036,
`module_game_menus.py` +766, `module_simple_triggers.py` +438 koymuş; ayrıca 305 satırlık `modmerger`
çerçevesi ve ~2.9k satırlık üçüncü taraf formasyon AI'sı taşıyor.

Kanıt yolu: mod klasörü gerçekte `mods/08-Money-and-Honour/src/` (tireli, `src/` alt klasörü); kit `kit/`.
"Mod ekledi" dediğim yerde kitin aynı dosyası okundu: kit `module_presentations.py:19` doğrudan
`game_credits` ile başlar, mod ise aynı listede önce 4 yeni ekran açar (`module_presentations.py:28,450,637,676`);
kit `module_variables.py` tek satırdır (`reserved_variables = []`).

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Every object type's persistent fields are hand-numbered integer slots collected in one constants file; "schema" = comments + reserved ranges | `mods/08-Money-and-Honour/src/module_constants.py:12-19`, `:130-151`, `:250-253` | ~40 | all modules; 5 object families |
| 2 | STATE | Cooldowns/deadlines are just timestamp slots ("last attacked hours", "next action time"); the engine supplies no timer object | `mods/08-Money-and-Honour/src/module_constants.py:149`, `:251-252` | ~4 | agent + faction state |
| 3 | TIMER | A 1-second simple trigger is the game's tick loop: it mutates a global counter and clamps it to a range | `mods/08-Money-and-Honour/src/module_simple_triggers.py:25-43` | ~19 | every tick |
| 4 | TIMER | Hand-rolled calendar from (check, delay, re-arm) tuples: 24h day tick, 0-re-arm month/year rollover, 6-hour date message | `mods/08-Money-and-Honour/src/module_triggers.py:52-60` | ~9 | 4 triggers share the idiom |
| 5 | TIMER | `ti_once` pass over every object at load to seed random per-object values (no "on create" hook) | `mods/08-Money-and-Honour/src/module_triggers.py:35-50` | ~16 | one pass over all centers |
| 6 | COLLECTION | Arrays emulated as strided slot ranges: `*_begin` constant + index arithmetic to find the parallel value | `mods/08-Money-and-Honour/src/module_constants.py:281-284`, `mods/08-Money-and-Honour/src/module_mission_templates.py:15007-15015` | ~14 | 4 array families + companion id/level pair |
| 7 | COLLECTION | Dummy troop records used as global arrays: overlay handle, type and name stored in parallel slot arrays, scanned linearly | `mods/08-Money-and-Honour/src/module_presentations.py:306-309`, `:566-580` | ~19 | center list, item grid, wave data |
| 8 | COLLECTION | Object ranges delimited by sentinel ids instead of a list type ("everything between troop A and troop B") | `mods/08-Money-and-Honour/src/module_constants.py:20-21` | ~2 | troop ranges |
| 9 | UI | Full-screen view built from overlays: pixel geometry, fixed-point multiplier, row/column loops projecting world coords to screen | `mods/08-Money-and-Honour/src/module_presentations.py:28-59`, `:256-310` | ~120 | 1 screen (`world_map`) |
| 10 | UI | Live HUD = read-only presentation whose `ti_on_presentation_run` recomputes values every frame and overwrites a text overlay | `mods/08-Money-and-Honour/src/module_presentations.py:637-669` | ~33 | 1 HUD (`killcount`) |
| 11 | UI | HUD readout bolted into an existing presentation: each line is created, coloured, positioned and sized by hand | `mods/08-Money-and-Honour/src/module_presentations.py:7962-7991` | ~30 | 1 HUD, patched into a Native screen |
| 12 | EVENT | Widget events dispatched by a hand-written if/else chain over the widget handle; each branch re-scans the whole array and toggles overlays | `mods/08-Money-and-Honour/src/module_presentations.py:417-447`, `:618-631` | ~35 | every widget of 2 screens |
| 13 | EVENT | Modal notification menus are the script→UI event channel; payload passed through globals, answer read back in menu options | `mods/08-Money-and-Honour/src/module_game_menus.py:15039-15070`, `:15109` | ~32 | several notification menus |
| 14 | TEXT | String ids computed by adding an offset to a base string id, stored into an `s` register, then referenced as a string id (`"str_s0"`) | `mods/08-Money-and-Honour/src/module_simple_triggers.py:66-70`, `mods/08-Money-and-Honour/src/module_mission_templates.py:15047-15055` | ~12 | 2+ call sites |
| 15 | INPUT | Held-key / click / mouse polling inside a per-tick mission trigger, then manual movement math per axis | `mods/08-Money-and-Honour/src/module_mission_templates.py:133-147`, `:268-272` | ~20 | 4 camera triggers |
| 16 | AGENT | Custom mission trigger on an agent event (dismount) reading trigger params and applying hand-written speed→damage math | `mods/08-Money-and-Honour/src/module_mission_templates.py:37-59` | ~23 | 1 trigger |
| 17 | NETWORK | Server→client custom event channels with int and string payloads; message text composed into a string register first | `mods/08-Money-and-Honour/src/module_mission_templates.py:15020`, `:15055-15059` | ~7 | 2 channels |
| 18 | AI | Third-party formation AI shipped as extra script files and injected into the `scripts` list by a merge adapter | `mods/08-Money-and-Honour/src/module_scripts.py:34-35`, `:56-62`, `mods/08-Money-and-Honour/src/formAI_scripts_wb.py:1-2` | ~35 + ~2.9k | all formation scripts |
| 19 | BUILD | Python code generator runs at import time and emits one operation tuple per item (data table → op list) | `mods/08-Money-and-Honour/src/module_scripts.py:25-32` | ~8 | 1 generator, all items |
| 20 | BUILD | A merge framework + per-file hook block inserted into every `module_*.py` so several mods can append to the same lists | `mods/08-Money-and-Honour/src/modmerger.py:1-48`, `mods/08-Money-and-Honour/src/module_variables.py:1-10` | ~315 | every `module_*.py` |
| 21 | CONFIG | Runtime probe of the toolchain version (try/except) to pick which merge routine to call | `mods/08-Money-and-Honour/src/module_scripts.py:42-60` | ~20 | each merge hook follows the idiom |
| 22 | DEBUG | Ad-hoc diagnostics: commented-out probe lines and inline `display_message` with `{regN}` left in triggers | `mods/08-Money-and-Honour/src/module_mission_templates.py:44-47`, `:57-58` | ~5 | inline, ad hoc |

## Top 5 things a language should give built in

1. Per-object persistent state as declared, typed fields (compiler-allocated slots), timestamps/cooldowns included.
2. First-class collections: lists, maps and ranges instead of strided slot ranges, sentinel ids and dummy-object arrays.
3. Declarative scheduling: tick, every N seconds, after-delay, once-at-load, per-object init — no re-arm tuples.
4. A UI/HUD framework: retained widgets with layout, per-widget state, event handlers, tooltips and live refresh.
5. Messages as data: language-level text formatting plus typed local and network events with payloads and a handler registry.

## Native code the mod changed

- `scripts`: formation AI merged into the Native script list, and the merge hook calls a routine chosen by toolchain version (`module_scripts.py:42-68`).
- `reserved_variables`: Native's one-line list replaced by a modmerger hook (`module_variables.py:1-10`; kit is one line).
- `presentations`: 4 new screens placed before Native's `game_credits` (`module_presentations.py:28,450,637,676`), and a wave/HUD block patched inside the Native multiplayer score table (`:7942-8020`).
- `simple_triggers`: a new 1-second stamina trigger added at the top of the list (`module_simple_triggers.py:25-43`).
- `triggers`: random per-center values, a manual calendar and a 6-hour date message inserted before Native's tutorial trigger (`module_triggers.py:35-60`).
- `mission_templates`: horse-fall damage and deathcam triggers added before Native's belfry trigger (`module_mission_templates.py:37-94`).
- `game_menus`: diplomacy notification/question menus and their `script_add_notification_menu` calls appended (`module_game_menus.py:15039-15070`, `:15109`).
- `module_strings.py`: HUD / co-op message strings added (+119 lines; e.g. `str_wave_info` used at `module_presentations.py:7962`).
