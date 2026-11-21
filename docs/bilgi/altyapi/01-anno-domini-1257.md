# Infrastructure scan: 01-anno-domini-1257

Total-conversion (historical) Warband mod: it replaces the whole Module System script layer in place
(the read-only copy holds the mod's `module_*.py` and `header_*.py` files, ~169k lines in the 16 files
scanned; the copy in this worktree has no `module_scripts.py`, so the script layer itself was not scanned). What it adds by hand is mostly
UI, data plumbing and combat scripting, because the Module System only offers slots, globals, string
registers and hour-interval triggers.

Evidence paths are relative to `target/ref/` (`mods/01-anno-domini-1257/...`). Native = the same file
line exists in `kit/` (Module System 1.171).

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Per-object persistent data lives in hand-numbered slots: one flat constants file assigns raw ids per object type, with reserved gaps, "reusable" ids and no name checking | mods/01-anno-domini-1257/module_constants.py:13, mods/01-anno-domini-1257/module_constants.py:69, mods/01-anno-domini-1257/module_constants.py:2069 | ~150 | ~300 slot reads/writes |
| 2 | TIMER | Periodic simple triggers: the interval is written in hours in the trigger tuple, and a "last time" global/slot is stored on fire to build day/week cooldowns | mods/01-anno-domini-1257/module_simple_triggers.py:533, mods/01-anno-domini-1257/module_simple_triggers.py:538 | ~20 | 5+ periodic triggers |
| 3 | TIMER | Deadlines are stamped hour/day values compared against the current clock on a daily tick (no timer objects) | mods/01-anno-domini-1257/module_simple_triggers.py:552, mods/01-anno-domini-1257/module_simple_triggers.py:571, mods/01-anno-domini-1257/module_simple_triggers.py:572 | ~50 | ~10 blockers |
| 4 | UI | A HUD is an overlay tree rebuilt from scratch on load: layout by decrementing a y cursor, scrolling via a scrollable container overlay plus per-row coordinates | mods/01-anno-domini-1257/module_presentations.py:15396, mods/01-anno-domini-1257/module_presentations.py:15405, mods/01-anno-domini-1257/module_presentations.py:15507 | ~120 | ~20 presentations |
| 5 | COLLECTION | Lists are runs of slots inside an existing record (`slot_*_begin` + fixed stride) or on the dummy troops `trp_temp_array_a/b`; indices are literals, holes and duplicate indices silently drop entries | mods/01-anno-domini-1257/module_presentations.py:13259, mods/01-anno-domini-1257/module_presentations.py:13268, mods/01-anno-domini-1257/module_presentations.py:13356 | ~80 | ~10 arrays |
| 6 | INPUT | Input is polled keys: `game_key_clicked` / `game_key_is_down` / `key_clicked` per frame, with a long `this_or_next` chain to mean "any of these keys" | mods/01-anno-domini-1257/module_mission_templates.py:1259, mods/01-anno-domini-1257/module_presentations.py:15557 | ~30 | pervasive |
| 7 | TEXT | Strings are built by stuffing values into numbered string registers s0..s71 and referencing them in template text (`{s1}`, `{reg1}`, conditional `{reg0?{s0}:none}`) | mods/01-anno-domini-1257/module_game_menus.py:3291, mods/01-anno-domini-1257/module_game_menus.py:16955 | ~15 | pervasive |
| 8 | TEXT | Choosing one of N pre-written lines is a hand-written if/else chain copying s1..s5 into s0, because there is no array of strings | mods/01-anno-domini-1257/module_presentations.py:15490, mods/01-anno-domini-1257/module_presentations.py:15506 | ~20 | 2+ UIs |
| 9 | CONFIG | Settings are plain globals set by a hand-built options presentation (checkbox per option, one `else_try` branch per widget); there is no persistence hook | mods/01-anno-domini-1257/module_presentations.py:14723, mods/01-anno-domini-1257/module_presentations.py:14911, mods/01-anno-domini-1257/module_presentations.py:14917 | ~600 | ~15 options |
| 10 | UI | Options screen: 14+ checkboxes with literal coordinates and per-widget overlay globals; adding a row means adding code in three places | mods/01-anno-domini-1257/module_presentations.py:14609, mods/01-anno-domini-1257/module_presentations.py:14720, mods/01-anno-domini-1257/module_presentations.py:14766 | ~500 | ~15 options |
| 11 | TIMER | Per-agent cooldowns are counters in agent slots, incremented/decremented by a fast mission trigger; the tick rate is encoded in the increment ("20 is 20*0.25=5 seconds") | mods/01-anno-domini-1257/module_mission_templates.py:1273, mods/01-anno-domini-1257/module_mission_templates.py:1680, mods/01-anno-domini-1257/module_mission_templates.py:1747 | ~20 | 2 features |
| 12 | TIMER | One timed behaviour is split over 9 triggers with different intervals (0.1 / 0.2 / 0.5 / 3 s) sharing one global flag; no coroutine or scheduler exists | mods/01-anno-domini-1257/module_mission_templates.py:1218, mods/01-anno-domini-1257/module_mission_templates.py:1229, mods/01-anno-domini-1257/module_mission_templates.py:1466 | ~300 | 1 feature |
| 13 | EVENT | Custom events are "notification menus": one call with a menu id and exactly two arguments, read back later from two globals by the handler menu | mods/01-anno-domini-1257/module_simple_triggers.py:545, mods/01-anno-domini-1257/module_game_menus.py:17713, mods/01-anno-domini-1257/module_game_menus.py:17718 | ~20 | many call sites |
| 14 | UI | Persistent HUD handles are parked in slots so a later frame can find and delete or redraw them | mods/01-anno-domini-1257/module_presentations.py:15461, mods/01-anno-domini-1257/module_presentations.py:15516 | ~10 | 1+ HUD |
| 15 | AGENT | Per-agent behaviour is a full `try_for_agents` scan inside a fast trigger instead of a per-agent callback or event | mods/01-anno-domini-1257/module_mission_templates.py:1231, mods/01-anno-domini-1257/module_mission_templates.py:1484 | ~50 | ~10 passes |
| 16 | STATE | Slot ids of unrelated Native records are aliased to "new" mod slots, so two features share one record's numbering to avoid renumbering the schema | mods/01-anno-domini-1257/module_constants.py:2064, mods/01-anno-domini-1257/module_constants.py:2065 | ~5 | 2 |
| 17 | COLLECTION | Array capacity inside a record is reserved by hand with a fixed stride (faction arrays start at 120 / 170 / 220 / 260) | mods/01-anno-domini-1257/module_constants.py:211, mods/01-anno-domini-1257/module_constants.py:212, mods/01-anno-domini-1257/module_constants.py:34 | ~40 | 4+ arrays |
| 18 | MATH | Rounding, clamping and percentages are written out with mul/max/div steps; the fixed-point multiplier is global state that every mission trigger resets | mods/01-anno-domini-1257/module_game_menus.py:17767, mods/01-anno-domini-1257/module_mission_templates.py:1230, mods/01-anno-domini-1257/module_mission_templates.py:1256 | ~10 | pervasive |
| 19 | TEXT | Numbers are shown by assigning to `reg0` and creating a text overlay from the literal string id `str_reg0` | mods/01-anno-domini-1257/module_presentations.py:15425, mods/01-anno-domini-1257/module_presentations.py:15426 | ~5 | pervasive |
| 20 | BUILD | Python-level code generation: repetitive menu entries are produced by a list comprehension over `range()`, and paged menus are chained by name | mods/01-anno-domini-1257/module_game_menus.py:16969, mods/01-anno-domini-1257/module_game_menus.py:16978 | ~30 | 4 pages |
| 21 | OTHER:trigger-bundle | Shared behaviour bundles are plain Python lists of triggers concatenated into many mission templates as a poor man's mixin | mods/01-anno-domini-1257/module_mission_templates.py:1607, mods/01-anno-domini-1257/module_mission_templates.py:4969, mods/01-anno-domini-1257/module_mission_templates.py:5640 | ~40 | 7 templates |
| 22 | OTHER:menu-navigation | Returning to a caller menu and paging a list are global variables written before each `jump_to_menu`; there is no call/return stack | mods/01-anno-domini-1257/module_game_menus.py:3394, mods/01-anno-domini-1257/module_game_menus.py:17869, mods/01-anno-domini-1257/module_game_menus.py:17916 | ~10 | ~10 menus |
| 23 | DEBUG | Developer tooling is built as extra menus: NPC export/import pages driven by hot keys, and cheat menus hidden behind a global flag | mods/01-anno-domini-1257/module_game_menus.py:16953, mods/01-anno-domini-1257/module_game_menus.py:749, mods/01-anno-domini-1257/module_game_menus.py:3435 | ~150 | ~6 menus |

## Top 5 things a language should give built in

1. Named per-object fields (slots) declared once with compiler-checked ids, no hand numbering, no aliasing, no reuse.
2. Real collections: lists/dicts of records with append/iterate, so no feature stores its state in slot ranges or dummy troops.
3. Scheduled work as a first-class concept: timers, cooldowns, deadlines and "every N seconds" jobs, instead of hour stamps and split triggers.
4. Declarative UI with layout, lists, scrolling, checkboxes and named input actions, instead of manual overlay coordinates and raw key polling.
5. Strings and events built in: formatting/interpolation without numbered string registers, and typed event dispatch without two global payload slots.

## Native code the mod changed

- Ships its own operator table (`mods/01-anno-domini-1257/header_operations.py`, 1734 lines vs the kit's 1902), i.e. it pins op ids instead of inheriting them.
- Native `camp` menu extended with mod options, lance disbanding, troop rebalancing and "resume travelling" entries (game_menus.py:3324, :3332, :3364).
- Native weekly payday/budget-report trigger extended with a `$g_last_payday` stamp and a deserters notice (simple_triggers.py:533; Native version at kit/module_simple_triggers.py:342).
- Native notification menus extended with diplomacy handler menus (`dplmc_deserters`, `dplmc_messenger`) (game_menus.py:17713, :17809, simple_triggers.py:5168).
- Native battle mission templates get the mod's trigger bundles appended (`must_1257_triggers + sp_shield_bash_triggers`) (mission_templates.py:4969, :5640).
- Native loot screen replaced by an item-pool menu pair with autoloot (game_menus.py:17839, :17938).
- Native presentations list extended with `mod_options` and `change_all_factions_color` (presentations.py:14609, :13245).
