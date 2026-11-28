# Mining: dialogs (`module_dialogs.py`)

Source set: the 17 read-only mods under `warband-opensource-mods/mods/`. Twelve ship dialog sources
(`02,03,04,05,06,07,08,09,10,14,15,17`); `05`/`10` ship an empty list. `01`, `12` and `13` import
`module_dialogs` but the file is missing from the dump (`01/process_dialogs.py:6`, `12/source/process_dialogs.py:6`,
`13/compile.py:135`), so `01` is cited only for headers, menus and triggers that drive dialogs.
All evidence is `mods/<folder>/<path>:<line>`.

## Patterns

| # | pattern (generic) | how it is done in MS | mods using it (of 17) | evidence |
|---|---|---|---|---|
| 1 | Dialog tree is a finite state machine over named state tokens | Each entry is a 7-field list; the entry's start state must equal the active state, and the chosen line's end state becomes the new active state. `close_window` terminates. There is no tree/edge object. | 10 — 02,03,04,06,07,08,09,14,15,17 | `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:22`, `mods/04-medieval-conquests/src/module_dialogs.py:4564`, `mods/04-medieval-conquests/src/module_dialogs.py:4570` |
| 2 | Player choices vs NPC lines; list order carries priority | All matching `plyr` lines at the active state are shown as options. For other speakers the first top-most match wins, so specific lines must be written before generic `anyone` lines. | 10 — 02,03,04,06,07,08,09,14,15,17 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:1640`, `mods/02-twilight-of-the-sun-king/module_dialogs.py:1642`, `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:8` |
| 3 | Speaker encoded as a bit mask | Constants are OR-ed: `anyone` (0xfff), `plyr` (0x10000), `party_tpl` (0x20000), `auto_proceed` (0x40000), `repeat_for_*` (0x1000-0x5000); `other(troop_id)` shifts the id by 20 bits for third-person lines. | 10 — 02,03,04,06,07,08,09,14,15,17 | `mods/01-anno-domini-1257/header_dialogs.py:15`, `mods/01-anno-domini-1257/header_dialogs.py:24`, `mods/01-anno-domini-1257/header_dialogs.py:29` |
| 4 | Whole line is a string register; speech is assembled in the consequence block | Text is `"{s21}"`; the previous line fills `s21` with `str_store_string` / `str_store_string_reg`, and the engine substitutes registers at display time. | 7 — 02,03,04,06,07,09,15 | `mods/07-brytenwalda/source/companions/questions.py:25`, `mods/02-twilight-of-the-sun-king/module_dialogs.py:1639`, `mods/04-medieval-conquests/src/module_dialogs.py:4570` |
| 5 | Engine-repeated option lists | `|repeat_for_troops/parties/factions/100/1000` makes the engine iterate a collection; the current object is fetched with `store_repeat_object` inside the condition and used to test/label the option. | 7 — 02,03,04,06,07,09,15 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:317`, `mods/02-twilight-of-the-sun-king/module_dialogs.py:319`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:2005` |
| 6 | One flat, position-addressed entry list | `dialogs = [` is a Python list of lists; fields are read by fixed index (speaker 0, start state 1, conditions 2, text 3, end state 4, consequences 5, voice 6). | 6 — 02,03,04,05,06,10 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:47`, `mods/03-tainted-paths/src/module_dialogs.py:46`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:50` |
| 7 | Verbatim "start" preamble re-storing conversation globals | A never-selected `[anyone,"start", ...]` entry first runs `store_conversation_troop`, `store_conversation_agent`, `store_faction_of_troop`, relation scripts and honorific string setup. The same block is pasted into every mod. | 6 — 02,03,04,06,09,15 | `mods/03-tainted-paths/src/module_dialogs.py:47`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:52`, `mods/09-between-empires/SourceCode/module_dialogs.py:48` |
| 8 | Per-NPC speech via string-id arithmetic | Compute the NPC index with `store_sub` (troop id minus `trp_npc1`), add it to a base string id with `store_add`, then `str_store_string` the result into a register used as the text. | 6 — 02,04,06,07,09,15 | `mods/07-brytenwalda/source/companions/questions.py:325`, `mods/04-medieval-conquests/src/module_dialogs.py:4567`, `mods/09-between-empires/SourceCode/module_dialogs.py:2889` |
| 9 | Display-time markup embedded in the text | `{!}` disables translation, `{reg65?my Lady:my Lord}` selects by gender/register, `{sir/my lady}` gives word variants, `{playername}` is a built-in; `@` prefixes a raw formattable literal. | 6 — 02,03,04,06,09,15 | `mods/03-tainted-paths/src/module_dialogs.py:925`, `mods/09-between-empires/SourceCode/module_dialogs.py:375`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:114` |
| 10 | Deliberately false condition `(eq,1,0)` as a setup/disable hook | Conditions are ordinary operation blocks evaluated during scanning, so a line whose last condition is `(eq,1,0)` can never be picked but its earlier `store_*`/`call_script` side effects still run; text is a `{!}Warning...` dummy. | 6 — 02,03,04,06,09,15 | `mods/04-medieval-conquests/src/module_dialogs.py:252`, `mods/04-medieval-conquests/src/module_dialogs.py:365`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:149` |
| 11 | Conversations opened from menus/triggers, not by clicking a party | `(start_map_conversation, <troop_id>[, <troop_dna>])` is called from game-menu items and simple triggers; the first dialog state is then `start`/`event_triggered`. | 6 — 01,02,04,06,09,15 | `mods/01-anno-domini-1257/module_game_menus.py:5499`, `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:2280`, `mods/09-between-empires/SourceCode/module_game_menus.py:4604` |
| 12 | Dialog data split into feature files / drop-in modules | Separate files each define or extend `dialogs` (`dialogs += [...]`), or ship a `modmerge(var_set)` that does `orig_dialogs.extend(...)`; the build then merges them into one list. | 4 — 07,08,14,17 | `mods/07-brytenwalda/source/companions/questions.py:9`, `mods/07-brytenwalda/source/companions/questions.py:330`, `mods/08-money-and-honour/src/freelancer_dialogs.py:192` |
| 13 | Speaker changed mid-conversation | Consequences call `set_conversation_speaker_troop` / `set_conversation_speaker_agent` to make another troop or scene agent speak the following lines. | 4 — 02,03,06,09 | `mods/03-tainted-paths/src/module_dialogs.py:11456`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:14902`, `mods/09-between-empires/SourceCode/module_dialogs.py:16703` |
| 14 | Random/variant speech chosen at runtime | Store 2-3 candidate lines, `store_random_in_range`, copy the picked register with `str_store_string_reg`, then show `"{s4}"`. | 2 — 02,09 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:1636`, `mods/09-between-empires/SourceCode/module_dialogs.py:1466`, `mods/09-between-empires/SourceCode/module_dialogs.py:1467` |
| 15 | `$talk_context` guard on every opener | Each encounter/tavern/court opener re-tests `$talk_context` against a `tc_*` constant so the same state only fires in the intended context. | 5 — 02,04,06,09,15 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:1631`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:153`, `mods/15-economy-mod/Module_system 1.134/module_dialogs.py:247` |
| 16 | Consequences leave the conversation through a screen/menu operation | `change_screen_map`, `change_screen_view_character`, `change_screen_equip_other`, `change_screen_trade_prisoners`, `jump_to_menu` are put in the consequence block instead of (or after) `close_window`. | 5 — 02,03,04,06,09 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:309`, `mods/04-medieval-conquests/src/module_dialogs.py:50`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:1242` |
| 17 | Position-aware overlay into an existing tree | Find the existing entry by start-state + end-state (and sometimes a text substring) and splice the new line at that index; helpers compute "insert before the last entry of this state". | 2 — 08,14 | `mods/08-money-and-honour/src/freelancer_dialogs.py:177`, `mods/14-dickplomacy-reloaded/source/cstmmerge_dialogs.py:120`, `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:85` |
| 18 | Nested operation blocks inside the 7 fields | Conditions/consequences may be written as `[[op],[op]]` or `[[op]]` instead of a flat `[op,op]`; the engine/compiler accepts both shapes. | 2 — 02,06 | `mods/02-twilight-of-the-sun-king/module_dialogs.py:1640`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:12841` |
| 19 | Empty dialog list, dialog exporter still run | `dialogs = []` is shipped and `process_dialogs.export(...)` is still called by the build, so dialog compilation is an independent step from the rest of the module system. | 2 — 05,10 | `mods/05-persistent-world/module_dialogs.py:41`, `mods/05-persistent-world/build_module.py:141`, `mods/10-persistent-kingdoms/module_dialogs.py:41` |
| 20 | Silent reroute at Native states with `auto_proceed` | A blank line (`{!}___`) marked `anyone|auto_proceed` at `start`/`member_chat`/`prisoner_chat` advances immediately into the mod's own `_00` subtree, intercepting Native flow without editing it. | 1 — 06 | `mods/06-the-last-days/ModuleSystem/module_dialogs.py:153`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:155` |
| 21 | Dual-engine dialog data built by Python list concatenation | The same file is compiled for two engines; whole dialog chunks are appended with `] + (is_a_wb_dialog and [...] or [])` keyed on a module-info switch. | 1 — 06 | `mods/06-the-last-days/ModuleSystem/module_dialogs.py:17`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:540` |
| 22 | Manual patch markers inside shipped dialog code | Comments such as "begin/end" and "insert the tagged code in the ... dialog entry" tell the user to hand-edit another mod's entry; the tagged block sits in the middle of a normal entry. | 1 — 04 | `mods/04-medieval-conquests/src/module_dialogs.py:5585`, `mods/04-medieval-conquests/src/module_dialogs.py:5586` |

## Engine facts learned

- An entry has 7 positional fields; the active state must equal its start state, and when the line is
  picked its end state becomes the active state (`close_window` ends the conversation).
  `mods/03-tainted-paths/src/module_dialogs.py:31`, `mods/03-tainted-paths/src/module_dialogs.py:41`
- The engine seeds five start states: `start` (map meeting / town talk), `party_relieved`,
  `prisoner_liberated`, `enemy_defeated`, `event_triggered`. `mods/03-tainted-paths/src/module_dialogs.py:32`
- Scan order is semantic: all matching player lines are listed, the first matching non-player line wins.
  `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:8`, `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:9`
- Condition blocks are full operation blocks evaluated while scanning, so their stores/side effects leak;
  a trailing `(eq,1,0)` makes a line unselectable while keeping it as a setup hook.
  `mods/04-medieval-conquests/src/module_dialogs.py:252`, `mods/04-medieval-conquests/src/module_dialogs.py:365`
- The start-state setup hook is the only place to initialize conversation globals before any line shows;
  six mods carry the same `store_conversation_troop` / relation block.
  `mods/03-tainted-paths/src/module_dialogs.py:47`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:52`
- Speaker flags are bit masks: `anyone=0xfff`, `plyr=0x10000`, `party_tpl=0x20000`,
  `auto_proceed=0x40000`, `multi_line=0x80000`, repeat flags `0x1000`-`0x5000`; `other(id)=id<<20`.
  `mods/01-anno-domini-1257/header_dialogs.py:15`, `mods/01-anno-domini-1257/header_dialogs.py:24`,
  `mods/01-anno-domini-1257/header_dialogs.py:29`
- `auto_proceed` plus blank text advances to the end state without user input, which is how a mod can
  intercept Native `start`/`member_chat` and jump into its own subtree.
  `mods/06-the-last-days/ModuleSystem/module_dialogs.py:153`, `mods/06-the-last-days/ModuleSystem/module_dialogs.py:155`
- `repeat_for_*` makes the engine iterate a collection; `store_repeat_object` returns the current object.
  `mods/02-twilight-of-the-sun-king/module_dialogs.py:317`, `mods/02-twilight-of-the-sun-king/module_dialogs.py:319`
- The compiler interns states against a small seed list (`start`, `party_encounter`, `close_window`,
  `trade`, `member_chat`, `prisoner_chat`, ...); a start state nobody produces is logged as
  "input token not found" and a produced state nobody uses as "output token not found".
  `mods/06-the-last-days/ModuleSystem/Process/process_dialogs.py:72`,
  `mods/07-brytenwalda/compiler/dialog.py:53`
- Text becomes a string id: spaces are replaced by `_`, empty text becomes `NO_TEXT`, and the auto id is
  `dlga_<in>:<out>` with `.1`, `.2` suffixes when two different texts collide.
  `mods/07-brytenwalda/compiler/dialog.py:120`, `mods/07-brytenwalda/compiler/dialog.py:150`
- Voice-over is optional and defaults to `NO_VOICEOVER`; mods almost always omit the 7th field.
  `mods/14-dickplomacy-reloaded/source/compiler.py:1288`
- A conversation can be opened from a game menu or a simple trigger, not only by clicking a party;
  the troop DNA argument is optional. `mods/01-anno-domini-1257/module_game_menus.py:5499`,
  `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:2280`
- The speaker can be switched mid-conversation with `set_conversation_speaker_troop` /
  `set_conversation_speaker_agent`. `mods/05-persistent-world/header_operations.py:1149`,
  `mods/03-tainted-paths/src/module_dialogs.py:11456`
- Returning to a hub state (`member_talk`, `lord_talk`, `*_pretalk`) is just another end state; this is
  the only "back" mechanism. `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:57`,
  `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:81`

## Pain points

- The `"start"` initializer is one ~150-200 line condition block; every mod reproduces it, and a bad
  merge in 02 swallowed a live `[anyone,"start", ...]` line into a comment.
  `mods/02-twilight-of-the-sun-king/module_dialogs.py:55`, `mods/04-medieval-conquests/src/module_dialogs.py:252`
- The honorific/gender "bug fix" block is pasted three times per file and copied across mods with the
  same `#bug fix` comment. `mods/06-the-last-days/ModuleSystem/module_dialogs.py:114`,
  `mods/09-between-empires/SourceCode/module_dialogs.py:156`,
  `mods/15-economy-mod/Module_system 1.134/module_dialogs.py:155`
- Per-NPC speech is encoded as contiguous string ids; adding an NPC requires editing the string order
  and every offset calculation. `mods/07-brytenwalda/source/companions/questions.py:324`,
  `mods/04-medieval-conquests/src/module_dialogs.py:4567`
- Option lists are hand-generated: the same filter and label-building operations are repeated in the
  `repeat_for_*` condition block for every menu. `mods/02-twilight-of-the-sun-king/module_dialogs.py:317`,
  `mods/15-economy-mod/Module_system 1.134/module_dialogs.py:308`
- No merge API exists, so three incompatible styles coexist: modmerger `extend` (08,17), index splicing
  (14) and copy-paste instructions (04). `mods/08-money-and-honour/src/freelancer_dialogs.py:192`,
  `mods/14-dickplomacy-reloaded/source/cstmmerge_dialogs.py:119`,
  `mods/04-medieval-conquests/src/module_dialogs.py:5585`
- Finding the right anchor is fragile: state+end-state is not unique, so 08 falls back to matching a
  text substring ("I have come", "I wish to become") to distinguish two Native lines.
  `mods/08-money-and-honour/src/freelancer_dialogs.py:180`
- Insert position is meaningful but implicit, so overlay helpers guess "before the last entry of this
  state" and silently append when the state is absent.
  `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:85`,
  `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:100`
- Side effects in conditions force the `(eq,1,0)` + "never displayed" dummy-line trick, which is
  invisible to readers and to any tooling. `mods/02-twilight-of-the-sun-king/module_dialogs.py:196`,
  `mods/04-medieval-conquests/src/module_dialogs.py:367`
- `$talk_context` is re-tested in every opener; some files use `tc_*` constants, others hard-code the
  integer 20. `mods/15-economy-mod/Module_system 1.134/module_dialogs.py:247`,
  `mods/04-medieval-conquests/src/module_dialogs.py:427`
- After a menu/screen round-trip `$g_talk_troop` is stale and must be re-stored inside a hub state.
  `mods/14-dickplomacy-reloaded/source/sort_dialogs.py:52`,
  `mods/09-between-empires/SourceCode/module_dialogs.py:192`
- Supporting two engine versions forces Python-level conditionals inside dialog data, so the dialog
  list is no longer a static table. `mods/06-the-last-days/ModuleSystem/module_dialogs.py:540`

## For UW

- Model a conversation as named nodes with explicit edges and fall-through; the compiler should emit
  the engine state tokens and statically reject unreachable nodes, unused states and unknown anchors.
- Make the speaker a typed value (`player`, `troop`, `party_template`, `anyone`, `agent`) so no author
  ever writes bit masks or `other(id)` shifts.
- Offer options as a data construct: "choices from <collection> where <expr> label <interpolation>",
  compiled to the repeat flag + repeat-object fetch instead of hand-written loops.
- Make speech interpolation an expression over typed values (names, relations, gender, pronouns, word
  variants) and a per-troop speech table, replacing string-id arithmetic and `str_store_string` chains.
- Generate the start-node preamble (talk troop/agent, relation, honorifics, context) once per entry
  node, so overlays never copy it and never collide on `"start"`.
- Keep conditions side-effect free and provide explicit `on_enter` / `on_pick` blocks, so the
  `(eq,1,0)` dummy-line hack has no reason to exist.
- Provide an overlay verb "extend <tree> at <node> before/after <line>" with stable line ids; the
  compiler computes the index and errors when the anchor is missing, instead of text matching.
- Treat translation opt-out and voice-over as attributes/annotations of a line, not prefixes inside
  the text, and expose display-time markup as a typed formatting feature.
- Allow opening a conversation from any verb/menu/trigger and switching speaker mid-tree as first-class
  operations, since dialogs are used as a general UI, not only as party talk.
