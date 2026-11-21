# Infrastructure scan: 09-between-empires

A 19th-century historical total conversion (firearms, field artillery, parliamentary politics) that ships a fully
merged Module System tree: ~935 scripts and ~50k lines of mod mission templates/presentations, most of the original
Native files rewritten in place rather than added as small modules.

Evidence paths are relative to `warband-opensource-mods/`. "Reused" is a rough count of call sites/instances.

| # | category | pattern (generic, one line) | evidence (file:line) | approx lines | times reused |
|---|----------|-----------------------------|----------------------|--------------|--------------|
| 1 | STATE | Per-entity state lives in hand-numbered engine slots; every feature claims its own slot ids on agents, props, troops, parties, items, factions and teams. | mods/09-between-empires/SourceCode/module_constants.py:411, 446, 610 | ~250 (const only) | 100+ slots |
| 2 | COLLECTION | Arrays are emulated as consecutive slots with `base + index` arithmetic; "reserve N slots" comments are the only allocator that stops collisions. | mods/09-between-empires/SourceCode/module_constants.py:638, 661 | ~60 | 10+ arrays |
| 3 | COLLECTION | Linked lists are emulated with next/previous slots written at scene-prop init, so scene props act as nodes. | mods/09-between-empires/SourceCode/module_constants.py:448; mods/09-between-empires/SourceCode/module_scene_props.py:3964 | ~40 | 36+ nodes |
| 4 | COLLECTION | Fixed-size ring buffers for history (a 100-entry price history spread over item slots, written through a "troop as array" hack). | mods/09-between-empires/SourceCode/module_presentations.py:19855, 19873-19875 | ~30 | 2+ buffers |
| 5 | TIMER | Cooldowns/timers are integers parked in slots or globals and decremented by a periodic trigger; there is no timer primitive or scheduler. | mods/09-between-empires/SourceCode/module_mission_templates.py:221, 2763 | ~200 | 20+ timers |
| 6 | INPUT | "Is a key held" is implemented by polling `game_key_is_down` every 100 ms and counting ticks in a global, reset on release. | mods/09-between-empires/SourceCode/module_mission_templates.py:24, 28-31 | ~26 | 2 keys |
| 7 | INPUT | Menus and HUDs poll `key_clicked`/`key_is_down` inside a per-frame presentation run handler; there is no general keyboard event callback. | mods/09-between-empires/SourceCode/module_presentations.py:47, 53-55 | ~200 | 40+ presentations |
| 8 | UI | Full-screen UI is raw overlay handles stored in global variables, positioned by hand in a 0..1000 coordinate space, with widget events handled by a switch on overlay ids. | mods/09-between-empires/SourceCode/module_presentations.py:8651-8652, 14640, 15422 | ~2000 | 40+ screens |
| 9 | UI | The battle HUD is one always-running presentation that a mission trigger starts and that updates every frame. | mods/09-between-empires/SourceCode/module_mission_templates.py:5838; mods/09-between-empires/SourceCode/module_presentations.py:10152 | ~1500 | 1 system |
| 10 | TEXT | Messages are built by writing values into numbered `s*`/`reg*` registers and formatting via `@...{reg1}...` templates; there is no argument passing or string concatenation operator. | mods/09-between-empires/SourceCode/module_presentations.py:20219; mods/09-between-empires/SourceCode/module_strings.py:13-15 | ~100 | 100+ strings |
| 11 | STATE | Transient cross-system state is a flat untyped global heap (`$g_*`, `$menu_*`, `$overlay_*`, `$player_*_timer`); any feature can read/write any of it. | mods/09-between-empires/SourceCode/module_mission_templates.py:29; mods/09-between-empires/SourceCode/module_presentations.py:8651 | ~2500 (uses) | 100+ globals |
| 12 | AGENT | Agents are bucketed into typed "companies" on spawn and per-company data is kept in parallel team slots addressed by `base + company index`. | mods/09-between-empires/SourceCode/module_mission_templates.py:5306, 936; mods/09-between-empires/SourceCode/module_constants.py:2311 | ~400 | 10 companies/team |
| 13 | AI | Battle AI is an explicit enum state machine (tactic id stored in a team slot) advanced by timed triggers. | mods/09-between-empires/SourceCode/module_constants.py:550-565, 2355 | ~80 (const) + logic | 17 states |
| 14 | EVENT | No event bus: each subsystem adds its own Native mission-template trigger for the same game event, and ordering is file position. | mods/09-between-empires/SourceCode/module_mission_templates.py:51, 68, 81, 5306 | ~500 | 20+ triggers |
| 15 | OTHER:time-sliced cursor | Whole-world loops are cut into chunks by a global cursor advanced each tick so the frame is not blocked. | mods/09-between-empires/SourceCode/module_simple_triggers.py:231-234 | ~120 | 8+ cursors |
| 16 | MATH | Math is integer/fixed-point by hand: the fixed-point multiplier is toggled per block and clamping is written with `val_clamp`; no float type exists. | mods/09-between-empires/SourceCode/module_mission_templates.py:304, 398; mods/09-between-empires/SourceCode/module_presentations.py:8640 | ~300 | everywhere |
| 17 | CONFIG | Feature toggles and debug switches are source constants that must be edited and recompiled; there is no runtime settings store. | mods/09-between-empires/SourceCode/module_constants.py:90-91, 332 | ~100 (const) | 20+ toggles |
| 18 | DEBUG | Tracing is done with commented-out `display_message`/`display_debug_message` lines plus per-subsystem debug flags. | mods/09-between-empires/SourceCode/module_constants.py:90; mods/09-between-empires/SourceCode/module_mission_templates.py:2766 | ~20 (flags) | 100+ traces |
| 19 | BUILD | A custom Python compiler on top of the Module System adds plugin registration, syntax extensions and opcode wrapping. | mods/09-between-empires/SourceCode/compile.py:54-55; mods/09-between-empires/SourceCode/compiler.py:755, 797 | ~900 | 1 build |
| 20 | BUILD | ModMerger: marker comments are spliced into Native module files by an installer so several mods can compose without a real module system. | mods/09-between-empires/SourceCode/modmerger.py:11-26; mods/09-between-empires/SourceCode/modmerger_options.py:11 | ~400 | dozens of markers |
| 21 | BUILD | Id tables (`ID_*.py`, 936 script ids, 2020 item ids) and compiled `.txt` outputs are code-generated and checked in. | mods/09-between-empires/SourceCode/ID_scripts.py:1; mods/09-between-empires/scripts.txt:2 | ~6000 generated | every id table |

## Top 5 things a language should give built in

1. First-class persistent records/objects with named fields on every entity kind, instead of hand-numbered slots.
2. Growable collections (lists/dicts) with indexing, insertion and iteration, instead of slot ranges, base+index math and hand-linked lists.
3. Timers, cooldowns and scheduled/periodic callbacks as language primitives, instead of tick counters kept in slots.
4. A retained UI/HUD layer with widgets, layout and events, instead of raw overlay handles parked in global variables.
5. A real module/import/build system with namespacing, composition and generated ids, instead of spliced Native files, merge markers and checked-in id tables.

## Native code the mod changed

- Game menus: extended the Native `camp_cheat` menu with mod simulation entries (`module_game_menus.py:2737`).
- Mission templates: merged mod triggers into Native battle templates (`module_mission_templates.py:5306`, `:1416`).
- Presentations: replaced/extended the Native presentation set and added a battle HUD (`module_presentations.py:10152`, `:14640`).
- Simple triggers: added time-sliced async cursor triggers (`module_simple_triggers.py:231`).
- Constants: extended the Native slot tables for items/troops/parties/teams (`module_constants.py:610`, `:638`, `:2311`).
- Scripts: recompiled the whole script set (935 scripts) over Native (`scripts.txt:2`).
