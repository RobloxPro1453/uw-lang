# Infrastructure scan - 11-diplado

`DIPLADO (DIPLomacy desperADO)` is a fork of the Diplomacy mod for Warband 1.174: a full Module System
source tree (`source/module_*.py`) that is roughly +38k lines over the plain 1.171 kit (dialogs +19.4k,
game menus +7.5k, presentations +5.3k, simple triggers +2.7k, mission templates +1.9k, constants/strings
+1.4k) and calls a few dozen `script_dplmc_*` / `script_diplomacy_*` helpers from its menus,
presentations, triggers and simple triggers. The copy has no `source/module_scripts.py`, so the helper
bodies themselves are not citable; all rows below come
from menus, presentations, triggers, constants and strings. Evidence paths are relative to
`target/ref/mods/11-diplado/`. Line counts below are rounded eyeball estimates, not exact totals.

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Extra fields on a game object: pick unused engine slots and alias them to readable constant names; slot numbers are hand-budgeted, shared between object types ("overlaps with ...") and deliberately reused. | `mods/11-diplado/source/module_constants.py:2066-2076`, `mods/11-diplado/source/module_constants.py:2090-2099`, `mods/11-diplado/source/module_constants.py:2155-2175` | ~200 | 40+ read/write sites; party, troop, faction, center and item slots |
| 2 | COLLECTION | Arrays: a fixed number of columns living in consecutive slots, walked with `try_for_range`; begin/end bounds and counts computed once in Python. | `mods/11-diplado/source/module_constants.py:2166-2175`, `mods/11-diplado/source/module_factions.py:80-82`, `mods/11-diplado/source/module_presentations.py:13934-13940` | ~80 | ~20 (trade-route arrivals, policies, item types, faction ranges) |
| 3 | UI | A screen is hand-built: every text/button/checkbox overlay created, sized and placed in fixed-point coordinates, overlay ids parked in `$g_presentation_obj_N` globals, clicks handled in `ti_on_presentation_event_state_change`. | `mods/11-diplado/source/module_presentations.py:13892-13940`, `mods/11-diplado/source/module_presentations.py:18101-18130` | ~5,300 added (delta vs kit) | 4 new screens (policy management, autoloot upgrades, companion overview, preferences) |
| 4 | CONFIG | Options page: N setting rows in a scrollable container, each row a label+control+icon triple kept in three parallel temp arrays; the count is a constant and in-game menus open the page. | `mods/11-diplado/source/module_presentations.py:18123-18130`, `mods/11-diplado/source/module_constants.py:2463`, `mods/11-diplado/source/module_game_menus.py:20039-20041` | ~450 | 12 options, 1 screen, reached from a dozen menu entries |
| 5 | TEXT | Text building: messages are assembled in string registers and every label needs a named quick-string id; real concatenation happens in Python at build time (`"..."+str(DPLMC_...)`). | `mods/11-diplado/source/module_presentations.py:13918`, `mods/11-diplado/source/module_game_menus.py:18762-18772`, `mods/11-diplado/source/module_strings.py:4304-4306` | ~120 | dozens of labels/messages (`module_strings.py:4302-4346`) |
| 6 | INPUT | Keys are polled from mission-template triggers (`key_clicked` / `key_is_down`), as a literal key or a key global; every action is its own trigger with its own check interval and conditions. | `mods/11-diplado/source/module_mission_templates.py:877`, `mods/11-diplado/source/module_mission_templates.py:1662`, `mods/11-diplado/source/module_mission_templates.py:773-778` | ~60 | 3+ actions (horse whistle, taunt, deathcam scroll) |
| 7 | TIMER | Time: either the trigger interval itself (how often it fires) or a counter stored in an agent/party slot and decremented on each tick; timestamps are built with `store_current_hours` minus a literal. | `mods/11-diplado/source/module_mission_templates.py:301-306`, `mods/11-diplado/source/module_mission_templates.py:315-339`, `mods/11-diplado/source/module_simple_triggers.py:614-617` | ~90 | ~10 (horse despawn, taunt cooldown, two-week grace, trade arrivals) |
| 8 | AGENT | Battle agents are managed by hand: spawn extra allied agents from the player's party into a scene entry point, re-team/re-align them when they appear, then cull or slow unmounted horses in a loop. | `mods/11-diplado/source/module_mission_templates.py:51-70`, `mods/11-diplado/source/module_mission_templates.py:142-158`, `mods/11-diplado/source/module_mission_templates.py:261-283` | ~250 | one shared trigger list, appended to several Native battle templates (e.g. `module_mission_templates.py:3815`) |
| 9 | AI | AI state is switched manually per agent: force rethink, clear/add relations, alarm flag, look and scripted destination, inside a `try_for_agents` loop. | `mods/11-diplado/source/module_mission_templates.py:1702-1716` | ~20 | 1 system, applied to every enemy in range |
| 10 | MATH | Arithmetic as op chains: percentage modifiers, clamps and skill-to-value ladders written as `val_mul`/`val_div`/`val_min` and if/else_try chains, because there are no expressions or lookup tables. | `mods/11-diplado/source/module_simple_triggers.py:607-609`, `mods/11-diplado/source/module_mission_templates.py:59-62`, `mods/11-diplado/source/module_mission_templates.py:1669-1699` | ~40 | several (income, bodyguard count, taunt range) |
| 11 | EVENT | Custom events: there is no event token, so a generic notification menu reads its payload from two globals (`$g_notification_menu_var1/2`) and the caller chooses the menu id. | `mods/11-diplado/source/module_simple_triggers.py:438-440`, `mods/11-diplado/source/module_game_menus.py:16615`, `mods/11-diplado/source/module_game_menus.py:16656` | ~30 | 40+ notification menus read these two globals |
| 12 | DEBUG | Debug surface: cheat-only menus that dump raw slot numbers for an object, plus version/code mismatch warnings printed from a slot value. | `mods/11-diplado/source/module_game_menus.py:3153`, `mods/11-diplado/source/module_game_menus.py:21058-21062`, `mods/11-diplado/source/module_game_menus.py:3470` | ~50 | 2 menus + warnings |
| 13 | NETWORK | Single-player and multiplayer paths must be written side by side: each battle trigger guards `multiplayer_is_server` / `game_in_multiplayer_mode` and keeps a separate SP branch. | `mods/11-diplado/source/module_mission_templates.py:264-266`, `mods/11-diplado/source/module_mission_templates.py:301-308` | ~10 | every battle-side trigger |
| 14 | BUILD | Repetitive menu options are generated by Python list comprehensions and `str(x)` names at build time, because the menu DSL cannot loop or parameterise a menu. | `mods/11-diplado/source/module_game_menus.py:4215-4224`, `mods/11-diplado/source/module_game_menus.py:4238-4247`, `mods/11-diplado/README.md:74-76` | ~20 | 2 paged menus; whole screen lists by hand otherwise |
| 15 | OTHER:patch-markers | Every edit to a Native file is wrapped in `##diplomacy begin` / `##diplomacy end` comments, so the mod stays a re-appliable patch set and reviewers see exactly what changed. | `mods/11-diplado/source/module_game_menus.py:44-46`, `mods/11-diplado/source/module_simple_triggers.py:9-13`, `mods/11-diplado/source/module_constants.py:2064-2076` | comments only | used in every edited Native file except dialogs |
| 16 | OTHER:save-version | Save compatibility: one integer version code plus a hand-written changelog of what each version changed in slot layout; on load the value kept in a troop slot is compared with the release code. | `mods/11-diplado/source/module_constants.py:2448-2463`, `mods/11-diplado/source/module_game_menus.py:20047-20061` | ~20 | 1 load-time check; long version changelog comment |

## Top 5 things a language should give built in

1. Named, typed fields on every game object, allocated by name instead of by a numeric slot budget.
2. First-class collections (list/map) with length, indexing and iteration instead of contiguous slot numbers.
3. A UI/options framework to declare a screen (rows, labels, controls, handlers) instead of placing overlays by coordinates.
4. Input bindings and timers/cooldowns as primitives (key event, "every N seconds", "after T") instead of trigger intervals and polled keys.
5. String formatting and interpolation over named text resources, instead of numbered string registers filled by hand.

## Native code the mod changed

- `module_game_menus.py`: options inserted into existing camp/action, town and peace menus (e.g. 16622-16628, 21058-21063).
- `module_simple_triggers.py`: Native lady-location trigger patched (270-279) and a notification call inserted (437-441).
- `module_mission_templates.py`: `dplmc_battle_mode_triggers` appended to Native battle templates (867, 3815, 3897) and `dplmc_horse_cull` added.
- `module_constants.py`, `module_strings.py`, `module_factions.py`: constants, strings and the faction list extended (factions list 76-83).
- `module_presentations.py`: ~+5.3k lines inside the existing presentation file.
- `module_dialogs.py`: extended 26.3k -> 45.7k lines, without patch markers.
- `module_triggers.py`: new world triggers added (1536-1554, 1601-1729).
- All of the above except `module_dialogs.py` carry `##diplomacy` begin/end markers.
