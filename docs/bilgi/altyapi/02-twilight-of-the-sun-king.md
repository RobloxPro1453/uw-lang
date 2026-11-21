# Infrastructure scan: 02-twilight-of-the-sun-king

Total-conversion mod (1702 Europe, Warband 1.153+) that ships a full Python Module System tree; `module_scripts.py` is 53,032 lines against the kit's 50,783 (~2.2k added lines, i.e. roughly a few dozen new scripts), concentrated in marked blocks (DrTomas cannons, `#LAV` companions overseer, MadVader deathcam, Mordechai EPC).

Evidence form: `mods/02-twilight-of-the-sun-king/<file>:<line>`, all read in `target/ref/mods/02-twilight-of-the-sun-king/`. Some rows cite code the mod placed in blocks of Native files; those blocks are listed at the end.

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Every object's data lives in a hand-numbered integer slot; one header allocates them and comments record renumbering and collisions | module_constants.py:635, :2187-2193, :2217 | ~80 | every system |
| 2 | COLLECTION | Fixed-capacity list emulated by a contiguous slot block plus a count slot; parallel slot arrays carry the attributes of the same elements | module_mission_templates.py:141-145, :179-182; module_constants.py:2192-2209 | ~40 | 3 (cannons, balls, scene props) |
| 3 | COLLECTION | Growable array kept in a dedicated object's slots (slot 0 = length, slots 1..N = handles) and rebuilt in place | module_mission_templates.py:134, :141-144; module_scripts.py:52953-52959 | ~25 | 2 (cannon handles, regiment handles) |
| 4 | TIMER | Per-object cooldown = a seconds counter decremented by an every-second mission trigger; at zero the object's state flips | module_mission_templates.py:333-343, :377-397 | ~40 | 2 trigger loops (reload, fire) |
| 5 | TIMER | Delayed effect scheduled by comparing the current world hour with a stored start hour and with start + duration | module_simple_triggers.py:4076-4099; module_constants.py:1864 | ~25 | 1 event (fire start/end) |
| 6 | TIMER | Recurring world-map job expressed as a simple-trigger interval (24 = hourly, 72 = every 3 h), with empty triggers reserved as placeholders | module_simple_triggers.py:4231-4239, :4191-4202 | ~15 | dozens of triggers |
| 7 | TIMER | One-shot setup / "switch off" by a constant-false condition or a `ti_once` flag inside a mission trigger | module_mission_templates.py:38-43, :562, :730-737 | ~10 | every mission template |
| 8 | INPUT | World-map hotkey polled every frame behind a "map is free" guard, then dispatched to a menu/presentation as a pseudo-event | module_simple_triggers.py:4209-4225 | ~20 | 1 (companions overseer) |
| 9 | INPUT | Held-key and mouse input polled per frame and converted by hand into movement/rotation deltas, with deadzone and a recenter hack | module_mission_templates.py:607-619, :659-690 | ~80 | 1 camera controller |
| 10 | MATH | Fixed-point setup plus manual abs/sqrt/clamp/sign to shape an analog input curve | module_mission_templates.py:705-712, :336 | ~10 | 2 |
| 11 | AGENT | Player death handled by finding another agent, copying its position and hit points, removing and respawning it as the new player troop, tagging both with slots | module_mission_templates.py:747-815; module_constants.py:2212-2213 | ~70 | 1 system |
| 12 | STATE | A game "object" modelled as a party whose identity is a set of slots (base troop, flag, officer, drummer, lord, HQ, size, type) | module_constants.py:2231-2241; module_scripts.py:52874-52899 | ~50 | all regiments |
| 13 | OTHER:multi-return | A call has no tuple/record result, so several outputs come back through reg0..reg4 and are read by position at each call site | module_mission_templates.py:59-61, :67-73; module_game_menus.py:14942-14948 | few per script | hundreds of call sites |
| 14 | OTHER:contract-comments | Each script documents its parameters and results only in free-text comments above it; nothing checks them | module_scripts.py:52855-52858, :52866-52869 | ~3 per script | hundreds of scripts |
| 15 | TEXT | Display text is assembled from s-registers and `{regN}` / `{sN}` placeholders inside menu and message strings | module_game_menus.py:15077, :15091-15093 | ~20 | dozens of menus/messages |
| 16 | UI | A screen is a presentation: clicks dispatch on global overlay ids, the flow chains `start_presentation` to the next page, and closes with duration 0 + menu jump | module_presentations.py:13420, :13585, :13605-13606 | thousands | 3+ screens (character creation, companions, credits) |
| 17 | DEBUG | A developer menu entry and `$cheat_mode`-gated trace messages shipped in the same build | module_game_menus.py:57-77; module_simple_triggers.py:4082-4089 | ~30 | 2 hooks |
| 18 | CONFIG | Tuning values and enum-like flags are Python constants, so changing them means rebuilding the module | module_constants.py:2150-2159, :2170-2180 | hundreds | every feature |
| 19 | BUILD | A Python code generator emits paged menus from a plain list, and import-time code post-processes other module lists | template_tools.py:33-110; module_game_menus.py:15116-15137 | ~120 | 2 generators |
| 20 | OTHER:storage-objects | Spare "dummy" objects (troops) used as heap memory: one holds data, one collects discarded handles | module_constants.py:2041-2042 | ~5 | 2 (storage, discard) |

## Top 5 things a language should give built in

1. Named, typed fields on game objects, checked at compile time, instead of hand-numbered integer slots shared by every system.
2. Growable typed collections with iteration and bounds checks, instead of fixed slot blocks or "length in slot 0" arrays.
3. First-class time: "after N seconds", "every N hours", "at hour H" — cancellable and attachable to an object.
4. Event-driven input: key press/hold and mouse motion delivered to a handler, with no per-frame polling or recenter hacks.
5. Formatted text and multi-value returns as language features, so no `{s}`/`{reg}` placeholder juggling and no reg0..reg4 conventions.

## Native code the mod changed

- `mnu_start_game_0` rewritten: new intro text, its "Continue" starts `prsnt_character_creation` instead of jumping to `mnu_start_game_1` (module_game_menus.py:39-56 vs kit module_game_menus.py:39-47).
- Character creation replaced by a chained presentation appended at the end of the Native presentation list (module_presentations.py:13420, :13611).
- Native slot table extended in place with `#TOTSK`, `## TEMPERED` and Mordechai EPC slot blocks (module_constants.py:634-643, :2026-2029).
- LAV companions-overseer hook: extra import in the Native presentation header, and a key trigger block appended to the Native simple triggers (module_presentations.py:11-17; module_simple_triggers.py:4205-4229).
- MadVader deathcam and auxiliary-player triggers appended to the Native mission templates and wired into a template list (module_mission_templates.py:560-729, :817-824).
- Native triggers/presentations edited in place, delimited by `#LAV MODIFICATIONS START/END` comments (module_simple_triggers.py:4206; module_presentations.py:12).
