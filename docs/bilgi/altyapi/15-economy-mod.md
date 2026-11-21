# 15-economy-mod — infrastructure scan (T-152)

**The mod in one line:** an economy-simulation overhaul (trade-good prices, production/consumption, enterprises, tariffs). Its source tree vendors a complete Module System 1.134 (`Module_system 1.134/`, >100k lines across its `module_*.py` files, `module_scripts.py` alone 46,964) and defines 568 scripts (`ID_scripts.py`).

**How "mod-added" was decided (kit diff):** against the plain 1.171 kit exactly one script is unique to this tree — `script_center_change_trade_good_production` (`ID_scripts.py:141`); 1.171 replaced it with `script_average_trade_good_prices_2`, moved the mod's inline merchant-refresh bodies into `script_refresh_center_inventories/armories/weaponsmiths/stables` and raised the refresh period from 24 h to 168 h. Rows marked **Δ** are where this tree's own code differs from the kit; unmarked rows are hand-written Module System Python of the same 1.134 vintage that the mod ships and maintains. Both kinds prove what the *language* lacks: the kit's copy is also hand-built code, not a language feature. All paths are relative to `target/ref/mods/15-economy-mod/`.

## Patterns

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Per-entity persistent fields: every town/party/item/troop gets a fixed numeric slot; slot ids are a hand-maintained global map with reserved ranges and prose comments. | mods/15-economy-mod/Module_system 1.134/module_constants.py:469, :522, :530 | ~80 declarations | 40+ slots, 5 files read/write |
| 2 | COLLECTION | Arrays emulated as contiguous slot blocks: entity id ↔ array index by subtracting the first id and adding a base constant. | mods/15-economy-mod/Module_system 1.134/module_scripts.py:13140-13141; mods/15-economy-mod/Module_system 1.134/module_triggers.py:48; mods/15-economy-mod/Module_system 1.134/module_presentations.py:11324 | ~4 per site | 10+ sites, 4 files |
| 3 | MATH | No float/pow/percent: ratio-to-the-third is hand-rolled as three integer mul/div pairs against a 1000-scaled factor. Δ | mods/15-economy-mod/Module_system 1.134/module_triggers.py:78, :81-86; mods/15-economy-mod/Module_system 1.134/module_constants.py:1557 | 6 | 1 hot loop (every good × town × day) |
| 4 | TIMER | Periodic world trigger: `(check, delay, period)` tuple with the period written inline; the same block is copy-pasted once per merchant class. Δ | mods/15-economy-mod/Module_system 1.134/module_triggers.py:38, :103, :151 | 1-3 per trigger | 4 refresh triggers + ~30 others |
| 5 | TEXT | Dynamic text only via string registers: each computed number is copied into `reg1..reg13` first, then one giant generated format key is stored into `s1`. | mods/15-economy-mod/Module_system 1.134/module_game_menus.py:10941-10961, :10966 | ~25 | every report line |
| 6 | UI | Report screen built by running nested loops in a menu consequence and concatenating the whole table into one string; no list/table widget. | mods/15-economy-mod/Module_system 1.134/module_game_menus.py:10891-10938, :10966 | ~80 | 2 reports (price/production, enterprise) |
| 7 | UI | Manual presentation layout: rows are created and placed one by one at pixel coordinates with a decrementing y cursor; no layout container. | mods/15-economy-mod/Module_system 1.134/module_presentations.py:11450-11486, :11488-11504 | ~30 per screen | 40+ `create_text_overlay` calls |
| 8 | STATE | Read-modify-write on a slot: get old value, add delta, optional clamp, write back; no field accessor and no save hook. | mods/15-economy-mod/Module_system 1.134/module_scripts.py:13143-13145; mods/15-economy-mod/Module_system 1.134/module_presentations.py:11324 | ~3 per site | 10+ sites |
| 9 | COLLECTION | Parallel arrays by index offset: merchant troop *i* is town *i* via `(troop - merchants_begin) + towns_begin`. Δ | mods/15-economy-mod/Module_system 1.134/module_triggers.py:107-108, :129-130, :155-156 | ~3 per loop | 3 merchant loops |
| 10 | TIMER | Deadline/cooldown: store an absolute game hour in a global, compare it with `store_current_hours` on a later trigger, then clear it. | mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:50-52, :114-120 | ~6 per site | 14 sites in simple_triggers |
| 11 | OTHER:multi-return | Multiple return values only through fixed registers `reg0/reg1/reg2`; the caller must copy them out before the next call. | mods/15-economy-mod/Module_system 1.134/module_scripts.py:13608-13610; mods/15-economy-mod/Module_system 1.134/module_game_menus.py:10898-10906 | ~3 per script | every multi-output script (~15 call sites) |
| 12 | MATH | Signed random delta: draw magnitude in a range, draw a 0/1 sign separately, add or subtract; used for stochastic state drift. Δ | mods/15-economy-mod/Module_system 1.134/module_scripts.py:13132-13139 | 8 | 2 scripts |
| 13 | INPUT | Key input: a per-frame presentation run-trigger tests `key_clicked` per key; the wheel is two keys; held key via `game_key_is_down`. | mods/15-economy-mod/Module_system 1.134/module_presentations.py:49-54, :6014-6021 | ~6 | 40+ key checks |
| 14 | UI | Widget dispatch: every clickable overlay id lives in its own global and is routed by a long if/else chain on object id + mouse state. | mods/15-economy-mod/Module_system 1.134/module_presentations.py:4059-4079 | ~20 per screen | dozens of objects |
| 15 | EVENT | Custom event bus via global flags polled by a trigger: a producer sets `$auto_menu`/`$auto_enter_town`, a map trigger jumps and clears it. | mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:123-127, :3427-3428 | ~8 | many quests/menus |
| 16 | CONFIG | Tuning numbers as module-level Python constants (price scale factors, adjustment percent); no config file and no options menu. Δ (NPC trades use the named constant where the kit hardcodes a literal) | mods/15-economy-mod/Module_system 1.134/module_constants.py:1557-1559, :1707; mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:2043 | ~5 | 6+ uses |
| 17 | DEBUG | Debug output selected by an integer cheat level; formatted debug strings are shown only when `$cheat_mode >= 1`. | mods/15-economy-mod/Module_system 1.134/module_game_menus.py:3129-3138, :10884-10888 | ~10 | 10+ sites |
| 18 | BUILD | Build-side config is a 7-line Python file holding only `export_dir`; everything else is Python code compiled by the Module System. | mods/15-economy-mod/Module_system 1.134/module_info.py:5 | 1 | 1 |

## Top 5 things a language should give built in

- Named, typed, persistent fields on every game entity (with automatic save/load), so nobody hand-numbers slots.
- Structured values: records and real collections (list/map, indexing, iteration, multiple return values), so nobody encodes arrays as slot blocks or keeps parallel arrays in sync.
- Real arithmetic: floats/fixed-point with `pow`, percent, clamp and signed random delta, instead of repeated integer mul/div.
- Declarative time: "every N hours", "at hour H", "after D", named cooldowns — persisted and re-armed by the runtime, not trigger tuples plus stored hours.
- Text and UI as data: interpolation into string values plus list/table widgets with layout, instead of register buckets, one giant format key, pixel cursors and one global per widget.

## Native code the mod changed

- `module_triggers.py:38-166` — the four "Refresh merchants" triggers stay inline with a 24 h period; the 1.171 kit replaced them with four `script_refresh_center_*` calls (kit `module_triggers.py:36-57`).
- `ID_scripts.py:141` — adds `script_center_change_trade_good_production`; the 1.171 kit has no such script (it uses `script_average_trade_good_prices_2` instead).
- `module_simple_triggers.py:2043`, `:2056` — caravan/farmer trades call `script_do_party_center_trade` with the named `price_adjustment` (25); the kit passes the literal `3`.
- `module_game_menus.py:5204`, `:5224` — `zendar` / `salt_mine` menus override item price rates; the same menus exist in the kit.
- `module_presentations.py:11284-11327`, `:11458-11499` — enterprise price editing and tariff/rent overlays; also in the kit.
- `module_constants.py:469`, `:522`, `:530` — trade-good slot layout; also in the kit (~7-10 lines later).
- `module_game_menus.py:10840-10984` — price/production report menu; also in the kit (shifted ~66 lines).

Uncertain: the mod tree is a full 1.134 copy, so "owns" here means "differs from the 1.171 kit"; it was not possible to tell 1.134-Native code from code the mod author edited without a pristine 1.134 tree. `module_scripts.py` is too large for text search, so its citations come from direct line reads.
