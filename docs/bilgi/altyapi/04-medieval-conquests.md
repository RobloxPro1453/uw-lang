# 04-medieval-conquests: infrastructure the Module System made the mod build by hand

What it is: a 13th-century-Europe campaign overhaul (about 240 000 lines of `module_*.py`; nearly every module file is a
rewritten Native file - presentations 38 455 lines vs 13 622 in the kit, strings 10 035 vs 4 342, game menus 42 111 vs
14 513) plus an optional layer that hosts the campaign as Co-Op/Multiplayer through WSE. That layer is a new
`module_coop_scripts.py` (6 373 lines, 30+ scripts) and `module_coop_mission_templates.py`, neither of which exists in
the kit.

Method: the read-only copy is `target/ref/mods/04-medieval-conquests/src/`; the plain kit `target/ref/kit/` (=
`Module_system 1.171/`) was used to tell Native from mod-added. Evidence cells are
`mods/04-medieval-conquests/src/<file>:<line>`; a second/third file in the same cell is in the same `src/` folder.
"approx lines" and "times reused" are reading estimates, not exact counts. The copy contains no `module_scripts.py`
(`process_scripts.py:4` imports it), so no script-internal pattern is claimed below.

| # | category | pattern (generic, one line) | evidence (file:line) | approx lines | times reused |
|---|----------|-----------------------------|----------------------|--------------|--------------|
| 1 | INPUT | Named-action registry: (variable, key code, label) tuples drive both the rebinding screen and the gameplay checks, so adding an action means editing three files | `mods/04-medieval-conquests/src/module_constants.py:2244`, `module_presentations.py:17137`, `module_mission_templates.py:45` | ~240 | 3 actions, ~10 uses |
| 2 | INPUT | Nothing is an action unless registered: raw key numbers inline in triggers (`game_key_is_down, 0..3`) and a global hotkey polled in a 0-second map trigger | `mods/04-medieval-conquests/src/module_mission_templates.py:3224`, `module_simple_triggers.py:5168` | ~20 | 40+ sites |
| 3 | EVENT | Hand-rolled event bus: an integer subtype enum plus one dispatcher script whose `else_try` chain routes (subtype, payload) to handlers; a new event touches enum, dispatcher and sender | `mods/04-medieval-conquests/src/module_constants.py:2127`, `module_coop_scripts.py:1671`, `module_constants.py:2191` | ~250 | 48 event ids |
| 4 | NETWORK | RPC by hand: pack a subtype plus 1-4 integers into `multiplayer_send_*_to_player/_to_server`; no return value, answers come back as further events | `mods/04-medieval-conquests/src/module_coop_scripts.py:1227`, `module_coop_scripts.py:323` | ~200 | 40+ call sites |
| 5 | STATE | Named map persisted outside the savegame: `dict_create/get_int/set_int/save/free` with `"@name"` keys (an engine extension - the plain kit has no `dict_*` operation) | `mods/04-medieval-conquests/src/module_coop_scripts.py:93`, `module_coop_scripts.py:97`, `module_coop_scripts.py:343` | ~15 | 30+ keys |
| 6 | UI | Modal options screen: one hand-placed overlay per setting and one `ti_on_presentation_event_state_change` branch per overlay that writes a global | `mods/04-medieval-conquests/src/module_presentations.py:14506`, `module_presentations.py:14969`, `module_presentations.py:15221` | ~1000 | 2 pages |
| 7 | CONFIG | Option round-trip: read engine `server_get_*` into globals, mirror them into `"@srvr_setN"` keys, reload next session; each option appears in three places | `mods/04-medieval-conquests/src/module_coop_scripts.py:364`, `module_coop_scripts.py:376`, `module_coop_scripts.py:436` | ~120 | 31 keys |
| 8 | CONFIG | Defaults registry: one "set defaults" script assigns every option global before use, so a new option needs a default here and an edit there | `mods/04-medieval-conquests/src/module_coop_scripts.py:479`, `module_coop_scripts.py:549` | ~90 | 20+ globals |
| 9 | STATE | Save-safe records in a dummy entity: key bindings and companion lists live in slots of troops that exist only as storage | `mods/04-medieval-conquests/src/module_constants.py:2279`, `module_constants.py:2291`, `module_presentations.py:17173` | ~10 | 3 dummy troops |
| 10 | COLLECTION | Arrays as contiguous slot blocks with begin/end constants computed by addition, several arrays packed into one entity | `mods/04-medieval-conquests/src/module_constants.py:2274`, `module_presentations.py:17169` | ~15 | 4+ arrays |
| 11 | COLLECTION | Index arithmetic instead of lookup: two parallel arrays in one block (i and i+n) and records found as base id + k*index over contiguous ids | `mods/04-medieval-conquests/src/module_presentations.py:19280`, `module_coop_scripts.py:703` | ~15 | 3 tables |
| 12 | COLLECTION | Reordering a container by copying it through a scratch container (clear, add back, twice) | `mods/04-medieval-conquests/src/module_coop_scripts.py:3869`, `module_coop_scripts.py:3888` | ~25 | 1 |
| 13 | TEXT | String building by repeated re-store: clear one register, then append piece by piece through the `"str_s2_s3"` template | `mods/04-medieval-conquests/src/module_presentations.py:3445`, `module_presentations.py:3451` | ~40 | dozens |
| 14 | TEXT | String tables made of contiguous generated ids (`"str_x1".."str_xN"`), reachable only by adding an offset to the first id | `mods/04-medieval-conquests/src/module_constants.py:2280`, `module_presentations.py:17169` | ~6 | 2 tables |
| 15 | TIMER | Countdown variable decremented by a 1-second trigger; reaching 0 fires the effect once | `mods/04-medieval-conquests/src/module_simple_triggers.py:7207`, `module_simple_triggers.py:7364` | ~15 | a few timers |
| 16 | TIMER | Day clock: a 1-second trigger keeps an hour counter in 1..24 and gates daily/hourly logic on it | `mods/04-medieval-conquests/src/module_simple_triggers.py:7101`, `module_simple_triggers.py:7151`, `module_simple_triggers.py:7160` | ~80 | ~10 checks |
| 17 | TIMER | Elapsed-time phase machine: `store_current_hours` minus a start global decides the phase | `mods/04-medieval-conquests/src/module_simple_triggers.py:3873` | ~8 | 1 |
| 18 | UI | Widget factory scripts returning overlay ids (mesh, label, button, container), so a large screen is built by calls instead of literals | `mods/04-medieval-conquests/src/module_presentations.py:19249`, `module_presentations.py:19251`, `module_presentations.py:19253` | ~40 | 1 screen, 100+ widgets |
| 19 | UI | Paged panel driven by a page-index global and rebuilt whenever the page changes | `mods/04-medieval-conquests/src/module_presentations.py:15159`, `module_presentations.py:19628` | ~80 | 2-3 screens |
| 20 | UI | Widgets are reachable later only through per-widget globals (`$g_presentation_obj_N`), so every screen invents its own global names | `mods/04-medieval-conquests/src/module_presentations.py:2598`, `module_presentations.py:2610` | ~100 | 20+ per screen |
| 21 | AGENT | Per-agent record fields as hand-numbered agent slots plus a constants block; duplicate and outlying numbers show the missing struct | `mods/04-medieval-conquests/src/module_constants.py:63`, `module_constants.py:99`, `module_mission_templates.py:53` | ~40 | 30+ slots |
| 22 | AI | Formation placement: loop over agents, match team and class, step a position per row/column, set scripted destination | `mods/04-medieval-conquests/src/module_coop_scripts.py:3451`, `module_coop_scripts.py:3485` | ~150 | 2 scripts |
| 23 | MATH | Integer-only percent, clamp and rounding arithmetic for sliders, gauges and map clicks | `mods/04-medieval-conquests/src/module_presentations.py:10468`, `module_presentations.py:4907`, `module_presentations.py:8566` | ~20 | dozens |
| 24 | DEBUG | Status-report menus and messages assembled from globals for in-game inspection | `mods/04-medieval-conquests/src/module_game_menus.py:3596`, `module_game_menus.py:3592`, `module_game_menus.py:3605` | ~30 | several |
| 25 | BUILD | Python-side composition: a trigger list defined once and concatenated into 39 mission templates, because modules cannot include each other's triggers | `mods/04-medieval-conquests/src/module_mission_templates.py:88`, `module_mission_templates.py:2126` | ~10 | 39 |

## Top 5 things a language should give built in

1. Collections and maps as real types (index, length, append, sort, iterate) that are saved with the game state.
2. Strings as values: concatenation, interpolation and string arrays without numbered ids or register juggling.
3. An input map: named actions with a generated rebinding UI and "clicked"/"held" queries, never raw key numbers.
4. A declarative UI toolkit: layout, widget handles, list/paging support, rebuild on change - not hand-placed overlays and global handles.
5. Scheduling and messaging as language features: timers, cooldowns and periodic jobs with remaining/elapsed time, plus a typed publish/subscribe event bus.

## Native code the mod changed

- `mnu_start_game_0` (Native's first menu) rewritten: new opening text and hints, mod `module_game_menus.py:41` vs kit `module_game_menus.py:39`.
- Native camp menu extended with new entries (`options_key`, `action_mod_options`); the kit's camp menu has neither: mod `module_game_menus.py:7600`, `:7610`.
- `game_multiplayer_admin_panel` overridden (the mod's copy differs at the header: `mesh_pic_bandits` where the kit passes `0`) and used as the mod's own settings panel: mod `module_presentations.py:2489` vs kit `module_presentations.py:1743`.
- Other Native presentations re-declared (same name in kit and mod, so the mod owns the definition): `game_credits` (mod 727 / kit 19), `game_profile_banner_selection` (986 / 278), `name_kingdom` (8024 / 8587), `marshall_selection` (9006 / 9575), `budget_report` (11698 / 12358).
- Simple triggers the kit does not have at all: disaster/hunger/hardcore cycles, `module_simple_triggers.py:7101`, `:7364` (kit `module_simple_triggers.py` has no such trigger).
- New, kit-less files: `module_coop_scripts.py` and `module_coop_mission_templates.py`.
- Script-side overrides cannot be checked: the copy has no `module_scripts.py`.
