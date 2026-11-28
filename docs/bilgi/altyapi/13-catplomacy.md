# 13-catplomacy — general infrastructure built by hand

A Warband diplomacy mod (Diplomacy 4.3 lineage) with a few smaller systems bolted on (a beer timer, autoloot, expanded battle formations); on top of Native it adds many `script_dplmc_*` / `script_diplomacy_*` call sites and roughly doubles several Native modules (game menus 14.5k -> 22.2k lines, dialogs 26.3k -> 45.7k), but the reference copy has no `module_scripts.py`, so how many scripts it adds cannot be counted exactly.

Evidence paths are relative to `target/ref/mods/13-catplomacy/` (`kit/` = `target/ref/kit/`). "approx lines" and "times reused" are estimates read off the reference copy.

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Persist per-object fields by writing raw integers into numbered engine "slots"; no records, no field names, uniqueness kept by hand in a constants file | `mods/13-catplomacy/module_constants.py:2669`, `module_constants.py:2673-2674`, `module_mission_templates.py:18410` | ~250 | 100s of sites |
| 2 | COLLECTION | Growable list emulated with three parallel dummy-troop slot arrays plus a manual index counter; init loop, append at the counter, counter bumped even for skipped rows to keep indexes aligned | `module_presentations.py:18125-18129`, `module_presentations.py:18153`, `module_presentations.py:18299` | ~600 | ~40 appends |
| 3 | COLLECTION | Multi-dimensional table emulated by arithmetic on the slot number: record = base + stride*index (8 battle divisions), matrix cell = partner + range_begin (with begin/end range constants) | `module_constants.py:2599-2605`, `module_constants.py:212-220`, `module_simple_triggers.py:6759-6760` | ~120 | every division/pair access |
| 4 | UI | Settings screen built row by row: scrollable container, one y-cursor decremented per row, label + input + picture overlays per entry, handles kept in the parallel arrays | `module_presentations.py:18108-18129`, `module_presentations.py:18146-18161`, `module_presentations.py:18171-18178` | ~800 | 1 screen, 12 rows |
| 5 | UI | Widget layer by hand: checkbox / combo / number-box overlays mapped to global variables, with manual clamping and value translation | `module_presentations.py:18156-18163`, `module_presentations.py:18187-18194`, `module_presentations.py:18461-18490` | ~80 | ~12 widgets |
| 6 | UI | In-mission HUD: a read-only presentation draws overlays on load and stores the handles in party slots so the key handler can animate or hide them later | `module_presentations.py:18897-18903`, `module_presentations.py:18946-18956`, `module_presentations.py:19045-19055` | ~170 | 1 HUD, 9 entries |
| 7 | INPUT | Key dispatch table: a long else_try chain maps game keys to small ints, then a range check plus slot lookup picks the ordered action | `module_presentations.py:19014-19031`, `module_presentations.py:19041-19047` | ~30 | 2 screens |
| 8 | INPUT | Poll a modifier key being held to expose a hidden shortcut inside a menu (normal click vs shift-click branch) | `module_game_menus.py:79-91`, `module_game_menus.py:21820` | ~20 | 3 menus |
| 9 | INPUT | Presentation close listener: a run trigger that only watches a fixed key set and ends the presentation | `module_presentations.py:18882-18893` | ~12 | a few screens |
| 10 | TIMER | Cooldown as a stored timestamp: keep "last time" in a slot, subtract current hours, compare against a limit | `module_constants.py:2673-2674`, `module_simple_triggers.py:6777-6782` | ~10 | 2-3 |
| 11 | TIMER | Periodic work declared as an interval number (sometimes an inline arithmetic expression with a comment) on a simple or world trigger | `module_simple_triggers.py:6768-6773`, `module_triggers.py:37-40` | ~15 | ~10 triggers |
| 12 | TIMER | In-mission timeout: read the mission timer, compare with a per-agent start slot, clear it and act when elapsed | `module_mission_templates.py:18404-18418` | ~15 | 2 |
| 13 | EVENT | No event bus: a trigger schedules a follow-up notification menu and calls the follow-up script in the same block, fire and forget | `module_simple_triggers.py:6622-6623` | ~2 | 1 seen |
| 14 | TEXT | String registers as scratch buffers: copy s1..s5 into s0 inside a loop, build the overlay text from s0, clear the registers afterwards | `module_presentations.py:18929-18946`, `module_simple_triggers.py:6588-6600` | ~50 | ~30 |
| 15 | TEXT | Sentence templates with positional placeholders in the string table, so code only fills registers and strings | `module_strings.py:16-27` | ~15 | many messages |
| 16 | MATH | Derived numbers computed once in Python (integer division) instead of at runtime, with the formula left visible | `module_constants.py:258-261` | ~5 | 4 |
| 17 | CONFIG | Every tunable is a named module constant inside a commented feature block; "use these constants instead of magic numbers" stated as a rule | `module_constants.py:235-261`, `module_constants.py:2498-2515` | ~300 | whole mod |
| 18 | STATE | Slot-space stretching: the same slot number carries several meanings, and mod fields are aliased onto Native party slots | `module_constants.py:197-202`, `module_constants.py:2544-2550`, `module_constants.py:2521-2522` | ~20 | ~30 aliases |
| 19 | STATE | Save/restore scratch registers around code inserted into a Native block that assumes its register values | `module_simple_triggers.py:6626-6629`, `module_simple_triggers.py:6661-6662` | ~10 | ~6 |
| 20 | AGENT | Per-agent transient state in agent slots (duel partner, duel start, volley flag, banner) instead of a typed agent record | `module_mission_templates.py:18409-18413`, `module_constants.py:2463`, `module_constants.py:2669` | ~30 | ~8 |
| 21 | DEBUG | Developer switches: a cheat flag assigned from a menu option, and register/position dump menus generated by code but left commented out | `module_game_menus.py:90`, `module_game_menus.py:22021-22040` | ~40 | 2 |
| 22 | BUILD | Python-side code generator: menus assembled by list comprehension plus `str()` concatenation | `module_game_menus.py:22029`, `module_game_menus.py:22077` | ~40 | 3 blocks |
| 23 | BUILD | Python-side checker that pairs every `*_begin` constant with its `*_end` and fails at import (commented out in the shipped copy) | `module_constants.py:2469-2492` | ~24 | 1 |
| 24 | OTHER:merge-markers | Every hand edit fenced by `##diplomacy start+` / `end+` (nested variant too) so upstream diffs can be rebuilt by hand | `module_presentations.py:10-15`, `module_simple_triggers.py:6582-6583`, `module_simple_triggers.py:6603` | 2 per hunk | dozens of hunks |

## Top 5 things a language should give built in

1. Declared, typed fields on every game object (troop, party, faction, centre, agent), with save-format handling generated from the declaration — no slot numbers, aliasing or clash hunting.
2. Real collections in the save format: lists, dictionaries and tables with append, index, length and iteration, so an array never has to be encoded in dummy objects or slot arithmetic.
3. Declarative UI: a screen described as rows of widgets bound to named variables; scroll containers, checkboxes, combos and number boxes are layout, not opcode sequences and overlay-handle juggling.
4. Timers and cooldowns as first-class values: run once after N, repeat every N, ask "elapsed since" — named intervals instead of bare numbers in a trigger tuple or a stored timestamp.
5. Events, handlers and scheduled follow-ups between features, plus interpolation of named values into messages, instead of string script names, fire-and-forget calls and string-register shuffling.

## Native code the mod changed

- `start_game_0`: menu text rewritten (`module_game_menus.py:43-46`) and a "Quick Character" option plus a hidden shift-key start added (`module_game_menus.py:51-96`).
- `start_phase_2`: Native consequence block extended with inventory auto-sort and duplicate-armour-to-gold (`module_game_menus.py:112-120`).
- Town sneak flow: Native menu replaced by `dplmc_choose_disguise` with six disguise options (`module_game_menus.py:21844`, `module_game_menus.py:21955-22003`).
- Rules inserted into a Native simple trigger and fenced with nested markers, saving/restoring reg0/reg1/reg4 (`module_simple_triggers.py:6582-6629`).
- Native presentation key-close listener reused in the mod's own settings screen (`module_presentations.py:18880-18893`).
- Mod ships its own `header_operations.py` (1896 lines vs the kit's 1902); its operation-list tail adds six operations, e.g. `agent_get_*_modifier`, `mission_tpl_are_all_agents_spawned` (`header_operations.py:1889-1894`; kit copy ends at `header_operations.py:1895-1901`).
- Most screens are new records rather than overrides: `adv_diplomacy_preferences`, `order_display`, `dplmc_choose_disguise`.
