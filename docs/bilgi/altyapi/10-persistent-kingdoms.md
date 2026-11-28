# Infrastructure scan: `10-persistent-kingdoms`

A multiplayer persistent-world mod built on the Persistent World (PW) Module System base. It adds
hundreds of `module_scripts.py` definitions (~15.5k lines), 27 presentations, and re-declares several
native engine hooks; nearly every reusable system below had to be hand-built because the engine lacks it.

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | EVENT | Hand-numbered custom network event ids split into server→client and client→server ranges, dispatched by one long `if/elif` chain on the native network hook | mods/10-persistent-kingdoms/header_common.py:7, mods/10-persistent-kingdoms/header_common.py:51, mods/10-persistent-kingdoms/module_scripts.py:1031 | ~1190 | one branch per event (~70 ids) |
| 2 | NETWORK | Manual state replication: each shared field is read from a slot and sent to each client as its own int message (no structured sync) | mods/10-persistent-kingdoms/module_scripts.py:177, mods/10-persistent-kingdoms/module_scripts.py:5904 | ~200+ | every synced player/entity field |
| 3 | STATE | Per-object persistent data kept in engine slots, with hand-assigned slot tables per entity type (player, agent, scene prop, troop, item, faction) | mods/10-persistent-kingdoms/module_constants.py:12, mods/10-persistent-kingdoms/module_constants.py:169, mods/10-persistent-kingdoms/module_scripts.py:128 | ~500 | used across all scripts |
| 4 | COLLECTION | Arrays emulated as a contiguous slot range: a count cell plus a `slot_array_begin`, appended by scanning for the first free slot | mods/10-persistent-kingdoms/module_constants.py:446, mods/10-persistent-kingdoms/module_scripts.py:7425 | ~20 per array | 10+ arrays |
| 5 | COLLECTION | Hidden dummy troops used as global arrays / buffers (one troop per list or ring buffer) | mods/10-persistent-kingdoms/module_troops.py:142, mods/10-persistent-kingdoms/module_troops.py:145, mods/10-persistent-kingdoms/module_troops.py:159 | ~22 | dozens of scripts |
| 6 | STATE | Boolean ownership flags ("is a key held") stored as player slots and mirrored to clients by the server | mods/10-persistent-kingdoms/module_constants.py:32, mods/10-persistent-kingdoms/module_scripts.py:177 | ~10 | 2 sync paths |
| 7 | UI | The whole menu/HUD layer rebuilt as presentations (27 of them) instead of engine menus | mods/10-persistent-kingdoms/module_presentations.py:1473, mods/10-persistent-kingdoms/module_presentations.py:2987 | ~5600 | 27 presentations |
| 8 | UI | Paged/scrollable list presentations built by hand: overlay ids kept in slots, page up/down recomputes rows | mods/10-persistent-kingdoms/module_presentations.py:1473, mods/10-persistent-kingdoms/module_presentations.py:1687, mods/10-persistent-kingdoms/module_presentations.py:1763 | ~200 each | 3+ lists |
| 9 | UI | Progress bars drawn as mesh overlays rescaled every presentation-run tick | mods/10-persistent-kingdoms/module_presentations.py:5343, mods/10-persistent-kingdoms/module_presentations.py:5388 | ~40 each | 2 bars |
| 10 | UI | Text entry emulated with a simple text-box overlay + focus + key handling (no engine input box) | mods/10-persistent-kingdoms/module_presentations.py:4258, mods/10-persistent-kingdoms/module_presentations.py:4293 | ~100 | 1 box, several callers |
| 11 | INPUT | Keyboard/control input via native mission-template key triggers, plus one gate that disables them while a modal presentation is open | mods/10-persistent-kingdoms/module_mission_templates.py:887, mods/10-persistent-kingdoms/module_mission_templates.py:892, mods/10-persistent-kingdoms/module_scripts.py:3308 | ~40 | ~20 key triggers |
| 12 | TIMER | Timers as mission-template polling triggers with a constant interval, and scheduled callbacks that scan a list for due times | mods/10-persistent-kingdoms/module_mission_templates.py:1092, mods/10-persistent-kingdoms/module_scripts.py:7422 | ~100 | many loops |
| 13 | TIMER | Per-action cooldown guard: last-action mission time in a slot compared against a fixed minimum interval | mods/10-persistent-kingdoms/module_scripts.py:2059, mods/10-persistent-kingdoms/module_constants.py:463 | ~6 | several actions |
| 14 | STATE | Reconnect persistence: disconnected players saved into an array keyed by the engine unique id and restored on rejoin | mods/10-persistent-kingdoms/module_scripts.py:4557, mods/10-persistent-kingdoms/module_scripts.py:4374 | ~120 | 2 scripts |
| 15 | OTHER:command-protocol | Console/chat command protocol: numeric command enum → long `if/elif` executor that validates and applies → generated human-readable description | mods/10-persistent-kingdoms/module_scripts.py:900, mods/10-persistent-kingdoms/module_scripts.py:2683, mods/10-persistent-kingdoms/header_common.py:117 | ~240 | all server settings |
| 16 | TEXT | Message styling encoded as a bitmask (colour/size/sound/log bits) packed into one int parameter | mods/10-persistent-kingdoms/header_common.py:94, mods/10-persistent-kingdoms/header_common.py:112 | ~30 | every message call |
| 17 | TEXT | String building by chaining one of the ~13 fixed string registers (`str_store_string*`), i.e. no real concatenation or interpolation | mods/10-persistent-kingdoms/module_scripts.py:1385, mods/10-persistent-kingdoms/module_scripts.py:1756 | pervasive | everywhere |
| 18 | TEXT | Chat overlay ring buffer: message text stored into the name slots of dedicated ring-buffer troops | mods/10-persistent-kingdoms/module_scripts.py:3296, mods/10-persistent-kingdoms/module_scripts.py:1393 | ~30 | 2 buffers |
| 19 | CONFIG | Options saved by packing booleans into the free bits of an existing engine field, with code-generated load/store | mods/10-persistent-kingdoms/module_constants.py:723, mods/10-persistent-kingdoms/module_scripts.py:4843 | ~40 | 10 options |
| 20 | BUILD | Build-time code generation: Python helpers emit operation lists from data tables into scripts and presentations | mods/10-persistent-kingdoms/module_presentations.py:300, mods/10-persistent-kingdoms/module_scripts.py:14357, mods/10-persistent-kingdoms/header_lazy_evaluation.py:5 | ~200 | dozens of call sites |
| 21 | DEBUG | Debug instrumentation framework: generated debug registers plus a live overlay presentation | mods/10-persistent-kingdoms/header_debug.py:62, mods/10-persistent-kingdoms/module_presentations.py:5576 | ~130 | ad-hoc |
| 22 | MATH | Build-time unit/id helpers and arithmetic between parallel id ranges | mods/10-persistent-kingdoms/module_constants.py:468, mods/10-persistent-kingdoms/module_constants.py:700, mods/10-persistent-kingdoms/module_scripts.py:13128 | ~15 | many |

## Top 5 things a language should give built in

1. **Typed records per game object.** Named, typed fields on players/agents/items instead of a flat, hand-maintained table of integer slot indices.
2. **Native collections.** Growable lists and key→value maps as values, so append/remove/lookup do not mean "slot range + scan for a free cell".
3. **Structured networking.** Typed messages and automatic state sync between server and clients, instead of a hand-numbered event registry and one send call per field.
4. **A declarative UI toolkit.** Lists, tabs, bars, text fields and world-space labels with layout and scrolling, instead of manual overlay ids, positions and a modal-input gate.
5. **A real string type with interpolation/localization.** Format and localize by name, not by chaining a dozen fixed registers and style bitmasks.

## Native code the mod changed

- Re-implements native script hooks: `game_receive_network_message` (mods/10-persistent-kingdoms/module_scripts.py:1031), `game_get_console_command` (:900), `game_get_cheat_mode` (:1029), `game_start` (:599), `game_quick_start` (:860), `game_set_multiplayer_mission_end` (:888).
- Empties ~30 other native hooks into stubs (`:898`, `:912`).
- Re-declares 5 native presentations, all also present in the plain kit (`target/ref/kit/module_presentations.py`): `game_credits` (mod :22 / kit :19), `game_profile_banner_selection` (:329 / :278), `game_custom_battle_designer` (:383 / :366), `game_multiplayer_admin_panel` (:385 / :1743), `game_before_quit` (:942 / :12987).
