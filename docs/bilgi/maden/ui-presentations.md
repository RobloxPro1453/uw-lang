# maden: ui-presentations (custom screens & HUDs)

Mined from the 17 mods under `warband-opensource-mods/mods/`. Every evidence path is relative to
that folder. `target/ref/kit/` is the reference for what is vanilla and what a mod added.

Where the presentations file lives differs per mod (root, `src/`, `source/`, `ModuleSystem/`,
`mod_src/`, `Module_system 1.134/`). Note `04-medieval-conquests` has **no** `module_presentations.py`:
its screens live in `src/module_coop_presentations.py`.

Counts say "mods where I confirmed the construct with at least one evidence line"; for very frequent
ops they are lower bounds (no full per-mod op census was done). `04, 09, 15, 16` are near-vanilla code
bases, so they mostly repeat the vanilla admin-panel / escape-menu / battle-order screens.

## Patterns

| # | pattern (generic) | how it is done in MS | mods using it (of 17) | evidence |
|---|---|---|---|---|
| 1 | Fixed-point canvas + position registers before any widget | Load trigger opens with `(set_fixed_point_multiplier, 1000)`; every coordinate is an int in 1/1000 of the screen. A layout is `position_set_x/y posN <int>` twice (position, then size) followed by `overlay_set_position` / `overlay_set_size` (or `overlay_set_area_size`) with that register | 10/17 verified (01,02,03,05,06,07,08,10,11,13) | `mods/02-twilight-of-the-sun-king/module_presentations.py:55`, `mods/05-persistent-world/module_presentations.py:3891`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:45` |
| 2 | Scrollable list region (the closest thing to a list widget) | `create_text_overlay reg1 "str_empty_string" tf_scrollable_style_2` → position → `overlay_set_area_size` sets the visible window → `set_container_overlay reg1`. Everything created afterwards attaches to it and is clipped/scrolled; `set_container_overlay -1` closes the region | 17/17 | `mods/05-persistent-world/module_presentations.py:3903`, `mods/05-persistent-world/module_presentations.py:3910`, `mods/03-tainted-paths/src/module_presentations.py:19278` |
| 3 | Widget value mirrored into a parallel global | There is no getter: a widget's value only arrives in `ti_on_presentation_event_state_change` as param 2 (`store_trigger_param_2 :value`). Mods test the object id, write the value into one `$g_presentation_obj_*` global per widget, then re-set the widget with `overlay_set_val` from that global | 4+/17 (05,06,10,13) | `mods/06-the-last-days/ModuleSystem/module_presentations.py:1902`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:1903`, `mods/10-persistent-kingdoms/module_presentations.py:1858` |
| 4 | Panel/background built from the white plane mesh | `create_mesh_overlay reg1 "mesh_white_plane"` + `overlay_set_color` + `overlay_set_alpha` + position/size = the panel; children are created afterwards, so z-order = creation order | 7+/17 (03,05,08,09,10,15,16) | `mods/05-persistent-world/module_presentations.py:3893`, `mods/10-persistent-kingdoms/module_presentations.py:1897`, `mods/08-money-and-honour/src/module_presentations.py:69` |
| 5 | Coloured button without art: image button on the white plane | `create_image_button_overlay obj "mesh_white_plane" "mesh_white_plane"` (same mesh for both states), then `overlay_set_color` / `overlay_set_size` — size and colour become data instead of a new texture | 4+/17 (07,09,15,16) | `mods/07-brytenwalda/source/battle/orders.py:37`, `mods/09-between-empires/SourceCode/module_presentations.py:10328`, `mods/16-warband-ai-chat/mod_src/module_presentations.py:10532` |
| 6 | Slider / number box as a generic option control | `create_slider_overlay dest min max` + `overlay_set_val` to show state + `overlay_set_boundaries` when the range depends on data; `create_number_box_overlay dest min max` for plain integers. The label beside it is a separate text overlay refreshed by hand | 3+/17 (01,13,14) | `mods/14-dickplomacy-reloaded/source/module_presentations.py:679`, `mods/13-catplomacy/module_presentations.py:14006`, `mods/01-anno-domini-1257/module_presentations.py:936` |
| 7 | Text list built by relying on consecutive overlay ids | Create N text overlays in a loop and keep only the first id (`store_add begin, container_id, 1`); refresh by walking ids with `val_add 1`. No add/remove API, so a fixed row count or a ring buffer is used | 3+/17 (05,06,13) | `mods/05-persistent-world/module_presentations.py:3912`, `mods/05-persistent-world/module_presentations.py:3920`, `mods/13-catplomacy/module_presentations.py:15754` |
| 8 | Fake arrays: widgets parked in troop slots | MS has no arrays, so overlay ids are stashed in `trp_temp_array_a/b/c` slots indexed by the loop counter, then read back later | 1+/17 (06) | `mods/06-the-last-days/ModuleSystem/module_presentations.py:6280`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:6286`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:6305` |
| 9 | Two-pass screen build (create all, then place all) | First loop creates widgets and records ids; a second loop recomputes y from the index, wraps to the next column, and sets position and value. Needed because the row count is only known at runtime | 1+/17 (06) | `mods/06-the-last-days/ModuleSystem/module_presentations.py:6313`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:6320`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:6327` |
| 10 | Screen generated from a Python table (meta-generator) | A Python `def` returns a `lazy.block` of ops built from a table, or loops in Python emitting `"%s_%d" % (...)` global names — the emitted MS is still flat code | 3+/17 (05,07,10) | `mods/05-persistent-world/module_presentations.py:300`, `mods/07-brytenwalda/source/battle/orders.py:24`, `mods/10-persistent-kingdoms/module_presentations.py:3723` |
| 11 | HUD lifetime expressed as duration | `presentation_set_duration 999999` at the end of the load trigger keeps the HUD up; `presentation_set_duration 0` from the run trigger hides it. There is no destroy-overlay op | 5+/17 (05,10,11,13,17) | `mods/05-persistent-world/module_presentations.py:3928`, `mods/05-persistent-world/module_presentations.py:3933`, `mods/13-catplomacy/module_presentations.py:15746` |
| 12 | HUD re-attached after another screen wipes it | The log window / full-screen menus clear all running presentations, so each battle-HUD piece is re-started from the mission template's `ti_battle_window_opened`, still guarded by its own display global | 2+/17 (05,11) | `mods/05-persistent-world/module_mission_templates.py:707`, `mods/11-diplado/source/module_mission_templates.py:1950` |
| 13 | HUD toggled from the mission template | Trigger polls `game_key_clicked` / `game_key_is_down` and starts the presentation only if `neg\|is_presentation_active` — the only way to ask whether a screen is up | 4+/17 (05,11,17,02) | `mods/11-diplado/source/module_mission_templates.py:1952`, `mods/17-nmc-enhanced-module-system/module_presentations.py:6120`, `mods/05-persistent-world/module_presentations.py:3932` |
| 14 | Per-frame refresh in the run trigger | `ti_on_presentation_run` gives elapsed time as param 1 (`store_trigger_param_1 :cur_time`); used as a debounce (`gt :cur_time 200`) and to redraw labels from globals every frame | 3+/17 (05,06,10) | `mods/05-persistent-world/module_presentations.py:1559`, `mods/10-persistent-kingdoms/module_presentations.py:1881`, `mods/10-persistent-kingdoms/module_presentations.py:374` |
| 15 | World-anchored HUD (nameplate / marker) | `agent_get_position` → `position_move_z` → `position_get_screen_projection` → `position_get_x/y` → `overlay_set_position`, plus a bounds test that moves the label off-screen; overlays are created lazily on first use and reused | 1/17 (05) | `mods/05-persistent-world/module_presentations.py:3333`, `mods/05-persistent-world/module_presentations.py:3339` |
| 16 | Text input box | `create_simple_text_box_overlay dest` (or `create_text_box_overlay` for multiline), prefill with `overlay_set_text`, then `overlay_obtain_focus` (focus ops are text-box only). The typed string arrives as `s0` and must be copied at once, because later string ops reuse `s0` | 10/17 (01,02,03,05,07,09,12,13,15,16) | `mods/05-persistent-world/module_presentations.py:3813`, `mods/13-catplomacy/module_presentations.py:15752`, `mods/12-beggars-diplomacy/source/module_presentations.py:1983` |
| 17 | Tooltips and item popups | `overlay_set_tooltip obj str` for inline help; item detail is a hit-test loop over slots (`overlay_get_position` + bounds compare), then `show_item_details`, guarded by a `$g_current_opened_item_details` global and closed with `close_item_details` | 2+/17 (07,13) | `mods/13-catplomacy/module_presentations.py:4211`, `mods/07-brytenwalda/source/module_presentations.py:6785`, `mods/07-brytenwalda/source/module_presentations.py:6788` |
| 18 | Modal stack: return to the caller screen | Each screen keeps a `$g_*_return_presentation` global; before ending itself it checks `neg\|is_presentation_active` on that id and re-starts it | 2+/17 (05,10) | `mods/05-persistent-world/module_presentations.py:1550`, `mods/10-persistent-kingdoms/module_presentations.py:1885` |
| 19 | Tab / variant switching by hiding widgets | All variants are created once; `overlay_set_display obj 1/0` shows the selected one. Positions and ids of the hidden widgets still have to be tracked by hand | 1/17 (13) | `mods/13-catplomacy/module_presentations.py:5922`, `mods/13-catplomacy/module_presentations.py:5974`, `mods/13-catplomacy/module_presentations.py:14013` |
| 20 | Hover feedback by animating colour/alpha | `overlay_animate_to_color` / `overlay_animate_to_alpha` in `ti_on_presentation_mouse_enter_leave`, with param 2 as 0/1 (leave/enter) | 2/17 (01,06) | `mods/01-anno-domini-1257/module_presentations.py:10349`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:1759` |
| 21 | Number → text without any formatting op | `assign reg1 <int>` then `str_store_string s1 "str_reg1"` (the engine's virtual `str_regN` strings) and `overlay_set_text` with `s1` | 2+/17 (06,13) | `mods/06-the-last-days/ModuleSystem/module_presentations.py:1865`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:483` |
| 22 | Raw numeric flags, position registers and key codes | `position_set_x 1 830` (numeric register instead of `pos1`), text flags as literals (`16`, `32772`), key codes as literals — shortcuts that make a screen unreadable | 1+/17 (03) | `mods/03-tainted-paths/src/module_presentations.py:19271`, `mods/03-tainted-paths/src/module_presentations.py:9041` |
| 23 | Screens split into extra presentation modules | Beside `module_presentations.py`, mods add their own presentation modules (`module_coop_presentations.py`, `KAOSPolitical_presentations.py`, `battle/orders.py`), each with its own globals | 3+/17 (04,07,08) | `mods/04-medieval-conquests/src/module_coop_presentations.py:117`, `mods/08-money-and-honour/src/KAOSPolitical_presentations.py:418`, `mods/07-brytenwalda/source/battle/orders.py:37` |
| 24 | Presentation metadata: flags + mesh background | An entry is `(id, flags, mesh, triggers)`: `prsntf_read_only \| prsntf_manual_end_only` stops the frame from closing it; a mesh id (`mesh_load_window`) supplies the window background instead of overlays | 2+/17 (10,13) | `mods/10-persistent-kingdoms/module_presentations.py:1893`, `mods/13-catplomacy/module_presentations.py:15630` |
| 25 | Dead experiments left in place | Commented-out overlays, `#TODO` and unused blocks stay in the file, so one screen carries several abandoned designs | 2+/17 (06,14) | `mods/06-the-last-days/ModuleSystem/module_presentations.py:6304`, `mods/06-the-last-days/ModuleSystem/module_presentations.py:6314`, `mods/14-dickplomacy-reloaded/source/module_presentations.py:20867` |

## Engine facts learned

- A widget's value can be set but never read back: `overlay_get_position` is the only getter in the op
  table, so all slider/combo state has to be shadowed in a global —
  `mods/05-persistent-world/header_operations.py:388`,
  `mods/06-the-last-days/ModuleSystem/module_presentations.py:1902`.
- Fixed point is per-screen, not global: `set_fixed_point_multiplier, 1000` is repeated as the first
  statement of every load trigger, even inside nested tutorial blocks —
  `mods/03-tainted-paths/src/module_presentations.py:1026`,
  `mods/07-brytenwalda/source/module_presentations.py:33`.
- Coordinates are screen space 0..1000 on both axes, so "sizes" are not pixels and scale with
  resolution; a typical panel is hardcoded x=0, y=553, w=825, h=186 —
  `mods/05-persistent-world/module_presentations.py:3905`,
  `mods/05-persistent-world/module_presentations.py:3907`.
- `set_container_overlay` is sticky: every overlay created after it attaches to that container until
  `set_container_overlay -1`, and children are clipped to the container's `area_size`, which is also
  how scrolling works — `mods/05-persistent-world/module_presentations.py:3910`,
  `mods/10-persistent-kingdoms/module_presentations.py:3813`.
- One existing overlay can be moved into a container with `overlay_set_container_overlay obj,
  container` (`-1` to reset), used to scroll a 3D item mesh together with its text list —
  `mods/10-persistent-kingdoms/module_presentations.py:3901`,
  `mods/05-persistent-world/header_operations.py:394`.
- A presentation lives until its duration runs out or it ends itself; `999999` is the mod idiom for
  "keep it up", `0` hides it immediately, and overlays die with the presentation (no destroy op) —
  `mods/05-persistent-world/module_presentations.py:3928`,
  `mods/05-persistent-world/module_presentations.py:3933`.
- Overlay ids inside one presentation come out consecutively and mods depend on it: the "first row"
  is stored as container id + 1 and rows are walked with `val_add` —
  `mods/05-persistent-world/module_presentations.py:3912`,
  `mods/05-persistent-world/module_presentations.py:3920`.
- Text-box content arrives as `s0` in the event trigger and `s0` is clobbered by the next string
  operation, so it must be copied immediately —
  `mods/13-catplomacy/module_presentations.py:15752`.
- `overlay_obtain_focus` "works for textboxes only" (`mods/05-persistent-world/header_operations.py:391`)
  and has to be called right after creating the box, otherwise the player must click it first —
  `mods/05-persistent-world/module_presentations.py:3813`.
- The mesh slot of a presentation entry is a real background: `mesh_load_window` gives the parchment
  window for free, while `0` means "draw everything with overlays" —
  `mods/13-catplomacy/module_presentations.py:15630`,
  `mods/10-persistent-kingdoms/module_presentations.py:1893`.
- Full-screen UI (log window, tabbed stats) kills running presentations; the only repair is to
  `start_presentation` again from `ti_battle_window_opened` —
  `mods/05-persistent-world/module_mission_templates.py:707`,
  `mods/11-diplado/source/module_mission_templates.py:1950`.
- Combo item indices are just the order of `overlay_add_item` calls, so the option list has to be
  built in the same order in two places —
  `mods/06-the-last-days/ModuleSystem/module_presentations.py:6287`,
  `mods/06-the-last-days/ModuleSystem/module_presentations.py:6294`.
- There is no "does screen X exist?" query; `is_presentation_active <id>` is it, and it is always
  used negated before starting a screen — `mods/05-persistent-world/module_mission_templates.py:702`,
  `mods/11-diplado/source/module_mission_templates.py:1953`.

## Pain points

- Every widget needs a hand-written global plus a branch in the event trigger; a screen with twenty
  controls has twenty globals and a long `try_begin` ladder —
  `mods/06-the-last-days/ModuleSystem/module_presentations.py:1904`.
- No list widget: rows are text overlays whose ids are assumed consecutive, inside a text overlay
  faked into a scroll region; adding or removing a row means rebuilding the screen —
  `mods/05-persistent-world/module_presentations.py:3912`,
  `mods/07-brytenwalda/source/module_presentations.py:11260`.
- No layout engine or anchors: every widget costs three to five ops of absolute literals, so a small
  design change means editing dozens of numbers —
  `mods/05-persistent-world/module_presentations.py:3905`.
- Changing page or variant often means ending and re-starting the presentation, so state has to be
  smuggled through globals — `mods/10-persistent-kingdoms/module_presentations.py:1885`.
- HUD lifetime is fought manually: duration hacks (`999999`), re-declaration after the log window,
  and a per-HUD "am I already shown?" global — `mods/05-persistent-world/module_presentations.py:3932`,
  `mods/05-persistent-world/module_mission_templates.py:707`.
- Numbers must be printed through the virtual `str_regN` strings, so text with several values needs a
  chain of `assign reg` + `str_store_string` before one `overlay_set_text` —
  `mods/06-the-last-days/ModuleSystem/module_presentations.py:1865`.
- Widget identity is a global named after the screen plus an index
  (`$g_presentation_obj_admin_panel_20`), so renaming a screen means renaming every global —
  `mods/09-between-empires/SourceCode/module_presentations.py:1976`.
- The vanilla battle-order / admin / escape screens are copy-pasted into almost every mod: the same
  `(create_listbox_overlay, "$g_presentation_obj_battle_10", "str_space", 0)` line appears verbatim in
  `mods/11-diplado/source/module_presentations.py:10827`,
  `mods/12-beggars-diplomacy/source/module_presentations.py:10826`,
  `mods/13-catplomacy/module_presentations.py:10826`.
- Magic literals creep in as soon as a modder is in a hurry: numeric position register `1`, flag
  `32772`, raw key codes — `mods/03-tainted-paths/src/module_presentations.py:9041`.

## For UW

- A `screen`/`hud` declaration should own its layout: named anchors and sizes in floating point, with
  fixed-point conversion and the `position_set_*` + `overlay_set_position/size/area_size` triples
  generated per widget.
- Widget state should be first-class (`slider.value`, `combo.current` readable like a variable); the
  compiler emits the shadowed global, the `overlay_set_val` and the event-trigger assignment.
- Offer a real list/scroll verb: rows from a collection, with the scroll container, clipping area and
  row id arithmetic generated — the user never touches overlay ids.
- Generate the container bookkeeping: opening/closing regions, the "everything created after this
  attaches here" rule, and reparenting of preview meshes are details the language should hide.
- HUDs should declare their re-attach policy ("stay during battle, survive the log window, hidden by
  default"); the compiler emits duration hacks, mission-template re-start triggers and
  `is_presentation_active` guards.
- Text input should be one verb with focus managed automatically, and the "copy `s0` immediately"
  trap should not be expressible.
- Number/format interpolation must be a language feature (no `str_regN` dance), and named flags should
  replace numeric literals for justification, scroll styles, alignment and keys.
- The per-frame hook should be a typed `update(dt)` with the elapsed time unpacked, plus debounce
  helpers, so timer globals disappear.
- Screens should be composable modules with a modal-return verb, so "restart the caller" boilerplate
  is not hand-written.
