# Infrastructure scan: 14-dickplomacy-reloaded

Dickplomacy Reloaded is a large "everything" overhaul (diplomacy, Custom Commander, companions overseer,
freelance soldier, tournament overhaul, character-creation suite, colour dialogs) on top of the Warband
Module System. The snapshot holds the base `module_*.py` files (dialogs, game menus, presentations, simple
triggers, mission templates, items, triggers, constants) plus 40+ feature plugin files
(`<feature>_scripts.py`, `<feature>_game_menus.py`, `<feature>_presentations.py`, ...): a `modmerger`
framework imports each active plugin's `modmerge_<component>` function and lets it splice itself into the
loaded module lists. All evidence below comes from those files: every `file:line` reads as
`mods/14-dickplomacy-reloaded/source/<file>:<line>`.

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | BUILD | Plugin merge: a loader imports each active plugin's `modmerge_<component>(orig_objects)` and the plugin appends or rewrites the loaded module lists; the game's own files stay the base. | modmerger.py:87-100; array_scripts.py:545; companions_presentations.py:1315 | ~40 glue lines per feature | 40+ plugin files |
| 2 | BUILD | Patch directives: a data table of (op, insert/replace, search mode, anchor op, new block) rewrites an already-loaded Native script. | freelancer_scripts.py:1006; freelancer_scripts.py:1028; freelancer_scripts.py:1041 | ~70 per file, ~200 framework | 5 directives plus 1 commented in one file |
| 3 | BUILD | Record wrappers: a Python class per module record (menu, trigger, presentation, script) with field get/set plus op-block InsertBefore/After. | util_wrappers.py:27; util_wrappers.py:81; util_wrappers.py:206 | ~330 | used by most merge plugins |
| 4 | BUILD | Menu generator: a Python class expands a list into a chain of paginated menus with next/previous/back options. | template_tools.py:8; template_tools.py:74-110; module_game_menus.py:23149-23160 | ~110 | 1 menu chain (scene chooser) |
| 5 | COLLECTION | Hand-built arrays: an "array" is a disabled dummy party whose slots hold length, owner, signature, version and elements; get/set/insert/remove/sort are scripts. | array_scripts.py:37-54; array_scripts.py:177; array_scripts.py:368 | ~530 | one script per array operation |
| 6 | COLLECTION | Scratch buffers: fixed dummy troops used as untyped temp arrays, cleared at the top of each use. | freelancer_scripts.py:816-818; freelancer_scripts.py:829; freelancer_scripts.py:841 | ~20 per use | several call sites |
| 7 | COLLECTION | Sorting by re-adding: move every stack through pre-spawned disabled temp parties in the wanted order, then move them back. | sort_scripts.py:15-42; sort_scripts.py:57-94 | ~190 | 2 scripts |
| 8 | STATE | Hand-numbered slot namespace: new troop/centre slot constants are picked by hand and neighbouring numbers "reserved" in comments, because slots are one global integer space. | tournament_constants.py:4-5; tournament_constants.py:62; tournament_constants.py:68; tournament_constants.py:73 | ~90 of constants | 25 troop slots, 3 centre slots |
| 9 | STATE | Dummy troops as singletons: one dummy troop per subsystem, its slots are the whole persistent state (widget ids, options, counters, saved items). | companions_constants.py:5-6; gpu_scripts.py:23-31; tournament_scripts.py:24 | ~30 | one per subsystem |
| 10 | STATE | Object-as-record: all fields of a long-running feature (dates, xp, target party, backed-up banner) live in the slots of one quest object. | freelancer_scripts.py:116-119; freelancer_scripts.py:133-139; freelancer_simple_triggers.py:28 | ~30 | 1 record, read by several files |
| 11 | UI | Widget factory: parameter scripts create and place buttons, labels, checkboxes, sliders, mesh images and scroll containers, because overlays have no layout. | gpu_scripts.py:23-31; companions_scripts.py:1429; companions_scripts.py:1506 | ~400 | ~25 widget scripts |
| 12 | UI | Overlay registry: every created overlay id is written into a fixed slot of a dummy troop, so later passes can find and update it. | companions_presentations.py:47-53; companions_presentations.py:108-114; gpu_scripts.py:77 | ~40 | most presentation screens |
| 13 | UI | Overlays addressed through generated global variables (`$g_presentation_obj_<n>`), with the numbers hard-coded in the code. | tournament_scripts.py:27; tournament_scripts.py:36-44 | n/a | dozens per screen |
| 14 | UI | Manual refresh pass: after any data change one script re-pushes every widget value from the data (there is no data binding). | tournament_scripts.py:20-45 | ~40 | every change event |
| 15 | CONFIG | Option table: each option is a tuple (widget type, params, label, from-data block, to-data block); Python builds the settings screen from it. | xgm_mod_options.py:12-27; xgm_mod_options.py:31-55; xgm_mod_options_presentations.py:68 | ~600 | one entry per setting |
| 16 | CONFIG | Dependent options: toggling one option walks a slot range and enables/disables the related widgets by hand. | tournament_scripts.py:367-408 | ~60 | 1 dispatch script, all options |
| 17 | INPUT | Modifier keys: every shift/ctrl/alt variant is a separate `key_is_down` test in the presentation trigger. | companions_presentations.py:386-387; companions_presentations.py:480 | ~10 per screen | 40+ tests in one file |
| 18 | INPUT | Drag and drop: press/release/move kept in globals plus a floating mesh image that follows the mouse, since no widget supports dragging. | companions_scripts.py:332-360; companions_scripts.py:364 | ~150 | 1 item-transfer screen |
| 19 | TIMER | Scheduled events: a periodic simple trigger writes absolute timestamps into slots, later passes compare current time against them. | freelancer_simple_triggers.py:24-28; freelancer_simple_triggers.py:65-69 | ~50 | 2 triggers, 1 schedule |
| 20 | EVENT | Named handler scripts (no event bus): menus, dialogs and patch hooks call `*_event_*` scripts by name. | freelancer_scripts.py:56; freelancer_scripts.py:147; freelancer_scripts.py:1038 | ~50 | several handlers |
| 21 | TEXT | Message composition: string registers plus generated format strings, and label lookup at runtime through a string-id offset in a register. | companions_scripts.py:35-37; companions_scripts.py:44; freelancer_scripts.py:843-844 | ~20 | most messages |
| 22 | MATH | Progression and odds: constant tables written into slots at start, then percentage and scaling math clamped step by step with val_min/val_max. | tournament_scripts.py:2318-2324; tournament_scripts.py:326; tournament_scripts.py:886-891 | ~120 | every payout and level calc |
| 23 | MATH | Derived stats by staging: compute a cap from a data tree, write through a dummy troop, compare, then clamp when applying. | cstm_scripts.py:476-488 | ~40 | every attribute and skill change |
| 24 | DEBUG | Debug-level constants gate `display_message` traces; the same flags also switch diagnostic messages inside library scripts. | tournament_constants.py:12; tournament_scripts.py:1224-1233; array_scripts.py:51 | ~60 of flags | several flags, grouped per subsystem |
| 25 | DEBUG | In-game overlay layout editor: a presentation that moves and resizes widgets live and prints their coordinates, used to author screens. | gpu_scripts.py:649-686 | ~120 | 1 editor for the mod's own screens |

## Top 5 things a language should give built in

1. Growable, typed collections (list, map) with sort/search/insert/remove as ordinary expressions, not as
   scripts over borrowed engine objects.
2. Declarative UI: widgets and option forms are declared with data bindings and a layout (dependencies
   included), so a data change updates the screen without a hand-written refresh pass.
3. Real variables and scoped records: fields belong to a declared object (per troop, party, quest), with the
   key space owned by the compiler instead of hand-numbered global slot constants.
4. An event and scheduling layer: named events, hooks on built-in scripts, and timers with absolute
   timestamps, so features subscribe instead of patching built-in code and polling every hour.
5. A module/patch system with expressions, strings and building blocks: compose strings and formulas inline,
   and extend an existing module by declaring "insert here", not by writing op blocks into op blocks.

## Native code the mod changed

- Native scripts patched by op-block insert/replace: `game_start`, `encounter_init_variables`, `enter_court`,
  `game_event_simulate_battle`, `abort_quest`, `change_troop_faction` (freelancer_scripts.py:1006-1041).
- Native vassalage simple trigger (interval 32) gets an extra condition (freelancer_simple_triggers.py:87-89).
- Native menus `town`, `center_manage` and the character-creation chain edited in place
  (cstm_game_menus.py:166-200, cstmmerge_game_menus.py:251-267); native presentations replaced by name
  (c3_presentations.py:3222, tournament_presentations.py:3239); siege and tournament mission template
  trigger lists replaced (freelancer_mission_templates.py:162-168, tournament_mission_templates.py:1091).
- Base module files are shipped pre-modified too: generated menus appended (module_game_menus.py:23140-23160).
