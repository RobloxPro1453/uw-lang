# Native patching — how mods change Native

Mining of the 17 open-source mods (`warband-opensource-mods/mods/`) for one topic: how a mod
changes what Native already has — copy-and-edit of Native scripts/menus/dialogs vs. adding new
entries, renumbering, removed Native content, how a mod marks its own changes, and the pain of
merging two mods. Reference for "what is Native": the plain kit at `target/ref/kit/` (e.g. the
Native `start_game_0` menu is `kit/module_game_menus.py:39`, and the same id sits at
`mods/01-anno-domini-1257/module_game_menus.py:39` with the mod's own text).
Citations below are `mods/<NN-name>/<path>:<line>`.

`start_game_0` survives in the menus file of 13 of the 17 mods (01,02,03,07,08,09,11,12,13,14,15,16,17);
05 and 10 are multiplayer rebuilds that start their lists from scratch. That alone says the dominant
strategy is: take Native's Python lists, keep them, edit inside them.

## Patterns

| # | pattern (generic, one line) | how it is done in MS (2-3 lines, operations named) | mods using it (count of 17 + names) | evidence (file:line) |
|---|---|---|---|---|
| 1 | Copy the whole Native `module_*.py` and edit it in place | Keep every Native tuple at its original position and rewrite texts / op blocks inside it; ids stay where Native put them because the compiler derives them from list order. No merge step, no marker — the file *is* the mod. | 13/17: 01,02,03,07,08,09,11,12,13,14,15,16,17 | mods/01-anno-domini-1257/module_game_menus.py:39, mods/15-economy-mod/Module_system 1.134/module_dialogs.py:22409, mods/12-beggars-diplomacy/source/module_game_menus.py:43 |
| 2 | Add own content as new tuples after the Native ones, never between them | An extra `(id, …)` tuple is appended after the Native block (or into the mod's own list), so nothing above shifts; all fields must be written out because the compiler has no defaults for a written entry. | 3/17 verified: 01,07,17 | mods/01-anno-domini-1257/module_items.py:958, mods/17-nmc-enhanced-module-system/module_constants.py:2368, mods/07-brytenwalda/source/constable/patrols.py:242 |
| 3 | Mark own changes with a begin/end comment pair | `# <name> start` … `# <name> end` (or `begin`/`end`, `##diplomacy start+`/`end+`) around an inserted block, plus author tags on single lines; purely for humans, the compiler ignores them. | 6/17 verified: 01,04,06,07,11,17 | mods/06-the-last-days/ModuleSystem/Header/header_troops.py:15, mods/17-nmc-enhanced-module-system/module_constants.py:2356, mods/04-medieval-conquests/src/module_constants.py:120 |
| 4 | Tag a repurposed Native entry instead of adding a new one | The Native id and slot are kept, only the mesh/text/ops are swapped; a `#NEW: was <old name>` comment records what it used to be so the next porter can find the divergence. | 6/17: 01,03,04,07,08,16 | mods/01-anno-domini-1257/module_items.py:962, mods/16-warband-ai-chat/mod_src/module_items.py:361, mods/08-money-and-honour/src/module_items.py:372 |
| 5 | Disable Native content by commenting it out | Whole Native menus / menu options / op blocks are prefixed with `#` or `####` and left in the file; nothing is deleted, so the id simply disappears from the compiled list. | 3/17: 01,06,17 | mods/01-anno-domini-1257/module_game_menus.py:411, mods/06-the-last-days/ModuleSystem/module_game_menus.py:519, mods/17-nmc-enhanced-module-system/module_game_menus.py:216 |
| 6 | Generate entries from a template instead of writing them | A small Python class holds text/op templates with `{…}` fields and a `generate_menus(list)` method; the result is concatenated onto the Native list (`game_menus += …`). The generator lives in a helper module. | 3/17: 01,02,14 | mods/01-anno-domini-1257/module_game_menus.py:20238, mods/02-twilight-of-the-sun-king/module_game_menus.py:15137, mods/14-dickplomacy-reloaded/source/module_game_menus.py:23160 |
| 7 | Own feature file + merge hook (`def modmerge(var_set)`) | Each feature ships `<feature>_<component>.py` with its own list and a `modmerge(var_set)` function; the framework installer appends a `try: from modmerger import modmerge; modmerge(var_set)` block at the bottom of the copied Native module file, which then receives the feature's list. | 3/17: 08,09,17 | mods/08-money-and-honour/src/module_game_menus.py:15274, mods/17-nmc-enhanced-module-system/freelancer_dialogs.py:192, mods/09-between-empires/SourceCode/module_game_menus.py:16028 |
| 8 | Name-anchored patching (find by id, insert/replace ops) | Helper wrappers implement `FindDialog`/`FindDialog_i` (search by partner/state/text), `InsertBefore`/`InsertAfter` on an op block and `list_find_first_match_i` on ids; the feature file then injects into the *Native* entry without copying it. | 1/17 verified: 08 (17 carries the same feature files) | mods/08-money-and-honour/src/util_wrappers.py:202, mods/08-money-and-honour/src/util_wrappers.py:218, mods/08-money-and-honour/src/util_wrappers.py:612 |
| 9 | Full replacement of a Native entry by id | Search the id in the Native list, then overwrite the element with the feature's fully rewritten tuple ("full replace entries"); guarded by `try/except` that prints which injection failed and re-raises. | 1/17 verified: 08 | mods/08-money-and-honour/src/KAOSPolitical_game_menus.py:2198, mods/08-money-and-honour/src/KAOSPolitical_game_menus.py:2204, mods/08-money-and-honour/src/KAOSPolitical_game_menus.py:2216 |
| 10 | Feature folder owning its lists, composed with `+=`/`extend` | Each feature package exposes `scripts`, `dialogs`, `simple_triggers` …; the top file rebuilds the module list by summing the packages (`scripts += a.scripts + b.scripts`). Used where the mod does not fork Native at all. | 3/17: 05,07,10 | mods/07-brytenwalda/source/economy/__init__.py:290, mods/05-persistent-world/module_scripts.py:25, mods/10-persistent-kingdoms/module_presentations.py:20 |
| 11 | Carry over another mod's merge block (stacking) | Second-hand feature files are pasted in verbatim, including their `modmerge` hook and their own begin/end markers; a file that starts "Floris Bank System begin" holds NMC's slots inside it, and the formAI merge file declares its hook at the same line as 08's copy. | 2/17: 08,17 | mods/17-nmc-enhanced-module-system/module_constants.py:2371, mods/17-nmc-enhanced-module-system/formAI_scripts.py:1240 |
| 12 | Call an "injector" script from inside copied Native blocks | Instead of branching inside the copied Native op block, a single `(call_script, "script_<name>_inject", …)` is dropped into the Native condition list, so the mod's logic lives in one script and the copy stays readable. | 1/17: 03 | mods/03-tainted-paths/src/module_presentations.py:11419, mods/03-tainted-paths/src/module_dialogs.py:12265, mods/03-tainted-paths/src/module_game_menus.py:107 |
| 13 | Copy Native mission-template trigger blocks into a new template | The Native arena melee triggers are lifted into a new `tpe_*` trigger list ("all stuff copied from the original arena_melee_fight"), leaving the Native template untouched; only the relevant triggers are carried. | 3/17: 08,14,17 | mods/08-money-and-honour/src/tournament_mission_templates.py:18, mods/14-dickplomacy-reloaded/source/tournament_mission_templates.py:18, mods/17-nmc-enhanced-module-system/tournament_mission_templates.py:18 |
| 14 | Edit the `header_*` file even though it says not to | Native headers are copied and extended: new opcodes pasted in `header_operations.py` (WSE/coop block), and culture flags added to `header_troops.py` although the file's banner says "DO NOT EDIT THIS FILE!". | 1/17 verified each: 04 (operations), 06 (troops) | mods/04-medieval-conquests/src/header_operations.py:1745, mods/04-medieval-conquests/src/header_operations.py:1748, mods/06-the-last-days/ModuleSystem/Header/header_troops.py:10 |
| 15 | Re-declare a Native entry id in a later feature file | A feature's own list re-declares `start_game_0` and is `extend`ed into the main list at the bottom of the file; the merge tail carries the warning "use this only if there are no replacements (i.e. no duplicated item names)". | 1/17 verified: 14 | mods/14-dickplomacy-reloaded/source/c3_game_menus.py:14, mods/14-dickplomacy-reloaded/source/c3_game_menus.py:367 |
| 16 | Encode a list-order contract in a comment | Where an index is used as data (element order = id), the list is frozen by a comment: "don't change the order", "Don't change the order of the following strings!". Nothing enforces it. | 6/17: 10,11,12,13,14,17 | mods/10-persistent-kingdoms/module_scene_props.py:4114, mods/11-diplado/source/module_strings.py:1861, mods/17-nmc-enhanced-module-system/module_strings.py:1893 |
| 17 | Keep the generated ID lists, and dead ids on purpose | `ID_*.py` is written by `process_*.py` from the list index; when a script is superseded the old name is kept as an alias at its old number instead of deleting it, so nothing below renumbers. | 1/17: 01 (all mods carry ID_*.py; 10's README explains them) | mods/01-anno-domini-1257/process_scripts.py:29, mods/01-anno-domini-1257/ID_scripts.py:556, mods/10-persistent-kingdoms/README.txt:12 |
| 18 | Keep stale backup copies of patched Native files | Superseded copies stay next to the live one as `-back.py` files or in an `animation back/` folder; there is no way to prove removal is safe, so nothing is deleted. | 1/17: 01 | mods/01-anno-domini-1257/MODULE_DATA/Flora_kinds-back.py:1, mods/01-anno-domini-1257/animation back/module_animations.py:1 |
| 19 | Pin the Module System version and hard-code the output path | The mod folder is named after the MS release (`Module_system 1.134/`), and `module_info.py` carries an absolute Windows `export_dir` pointing at the author's machine. | 2/17: 01,15 | mods/01-anno-domini-1257/module_info.py:5, mods/15-economy-mod/Module_system 1.134/module_dialogs.py:22409 |
| 20 | Extend the Python compiler itself instead of forking it | A plugin file does `from compiler import *` + `register_plugin()`, optional modules get defaults, and a component can be switched off instead of edited. | 2/17: 13 (active), 11 (fallback variant) | mods/13-catplomacy/plugin_ms_extension.py:1, mods/13-catplomacy/defaults/module_item_modifiers.py:1, mods/11-diplado/source/compile.py:60 |
| 21 | Record licence scope of the patched Native base | A separate text file states that the Native MS licence is unknown and that the mod's own licence covers only its diff — the patch-based model has no per-line provenance to lean on. | 2/17: 05,10 | mods/05-persistent-world/LICENSE_explanation.txt:1, mods/10-persistent-kingdoms/LICENSE_explanation.txt:3 |
| 22 | Comment a copied block with where it came from | Small provenance notes inside copied code ("Copied from module_game_menus, removed the checks", "copied from below") so a later porter knows which lines are Native and which are the mod's. | 2/17 verified: 06,08 | mods/08-money-and-honour/src/yael_joinfreely_game_menus.py:22, mods/06-the-last-days/ModuleSystem/module_scripts_form_v5.py:1684 |

## Engine facts learned

- **Ids are list positions, not names.** `ID_*.py` is generated by looping over the final list and
  writing `name = index`; the mod's own README says the ID files "just list the numbers"
  (`mods/01-anno-domini-1257/process_scripts.py:29`, `mods/10-persistent-kingdoms/README.txt:12`).
  Same for animations (`mods/01-anno-domini-1257/process_animations.py:52`).
- **Appending is cheap, inserting is not.** New content is always added at the end of a list or via
  `+=` of a generated list (`mods/01-anno-domini-1257/module_game_menus.py:20238`); a removed script
  leaves an `_old` alias behind at its old number (`mods/01-anno-domini-1257/ID_scripts.py:556`).
- **String ids are positional too**, and scripts hard-code the `s42` index, so the string table order
  becomes an API: "Don't change the order of the following strings! (Refer to
  script_get_relevant_comment_to_s42 …)" (`mods/11-diplado/source/module_strings.py:1861`).
- **Scene-prop order is load-bearing**: the file itself says "[!] don't change the order"
  (`mods/10-persistent-kingdoms/module_scene_props.py:4114`, again at `:4121`).
- **Duplicate entity names are allowed by the source format** — a later list can re-declare an id; the
  merge tail that concatenates such a file warns that `extend` is only safe when there are no
  replacements / duplicated names (`mods/14-dickplomacy-reloaded/source/c3_game_menus.py:367`).
- **Troop type is a 4-bit field**: the Native header's `troop_type_mask` is `0x0000000f`, with
  hero/inactive/unkillable in the higher bits. A total conversion that wants 16 cultures has to edit
  that "DO NOT EDIT" header and fit its values into 0..15
  (`mods/06-the-last-days/ModuleSystem/Header/header_troops.py:16`, `:42`).
- **Operations are declared in the source tree, not the engine**: mods paste new op definitions in a
  "COOP BEGIN WSE" block and re-enable commented-out Native ops, then also uncomment the call sites
  in the mission templates (`mods/04-medieval-conquests/src/header_operations.py:1748`, `:1745`, `:558`).
- **The compiler is Python 2** (`print "…"`, `xrange`) so every tool that touches MS source is Python 2
  (`mods/01-anno-domini-1257/process_scripts.py:34`, `:28`).
- **There is no build-level dependency graph.** Processing order is a hand-written list of module
  names, with per-component overrides if the order differs
  (`mods/17-nmc-enhanced-module-system/modmerger_options.py:26`, `:53`).
- **Optional module files need defaults**: the compiler tries `from module_item_modifiers import *`
  and falls back to a built-in default list, even disabling generation of that component
  (`mods/11-diplado/source/compile.py:60`), and a mod can hook the compiler itself
  (`mods/13-catplomacy/plugin_ms_extension.py:1`).
- **Names are resolved only at compile time**: Tainted Paths calls `script_temp_save_number11_inject`
  in dozens of places but ships no definition of it (only the generated id line), so a missing script
  is invisible until the build (`mods/03-tainted-paths/src/module_dialogs.py:12265`,
  `mods/03-tainted-paths/src/ID_scripts.py:336`).

## Pain points

- **Every Native change is a re-merge.** 13 mods hold a forked copy of the Native lists, so a Native
  fix must be re-applied by hand against their file; nothing tells them which lines are theirs
  (`mods/01-anno-domini-1257/module_game_menus.py:39`, `mods/15-economy-mod/Module_system 1.134/module_dialogs.py:22409`).
- **Marking is manual and inconsistent.** Begin/end pairs, `#NEW:`, `#tom`, `####`, TLD blocks — each
  author invents a scheme, and markers of different mods end up nested inside each other
  (`mods/17-nmc-enhanced-module-system/module_constants.py:2356`, `mods/06-the-last-days/ModuleSystem/Header/header_troops.py:15`).
- **Replacing an entry means duplicating it.** The override path is "write the whole tuple again and
  hope your copy of the Native op block is current"; the merge helper's own comment admits duplicated
  names are the failure mode (`mods/08-money-and-honour/src/KAOSPolitical_game_menus.py:2204`,
  `mods/14-dickplomacy-reloaded/source/c3_game_menus.py:367`).
- **Merging is a framework plus glue.** To combine two mods you paste `modmerger.py`, paste an
  installer-generated `modmerger_start … modmerger_end` block into every `module_*.py`, and keep an
  ordered `mods_active` list; a leftover commented marker shows how often this half-works
  (`mods/08-money-and-honour/src/module_game_menus.py:15270`, `mods/07-brytenwalda/source/module_items.py:3767`).
- **Failures are silent or fatal.** Injections are wrapped in `try/except` that only prints and
  re-raises, so a renamed Native id stops the build with a line number of the framework, not of the
  mod (`mods/08-money-and-honour/src/KAOSPolitical_game_menus.py:2216`).
- **Order contracts live in comments** and are broken silently; a re-sorted list changes behaviour
  without any error (`mods/10-persistent-kingdoms/module_scene_props.py:4114`).
- **Disabling Native content leaves dead code** that still has to be kept compiling and read
  (`mods/01-anno-domini-1257/module_game_menus.py:411`, `mods/17-nmc-enhanced-module-system/module_game_menus.py:216`).
- **Removal is unprovable**, so superseded files and dead hook functions are archived next to the live
  ones (`mods/01-anno-domini-1257/MODULE_DATA/Flora_kinds-back.py:1`,
  `mods/07-brytenwalda/source/module_items.py:3735`).
- **Portability is broken by the setup itself**: absolute `export_dir` and version-named folders
  (`mods/01-anno-domini-1257/module_info.py:5`, `mods/15-economy-mod/Module_system 1.134/module_dialogs.py:22409`).
- **Provenance/licensing is untrackable per line**: the Native base has no clear licence, so mods can
  only license "their changes", which the tools cannot delimit
  (`mods/05-persistent-world/LICENSE_explanation.txt:1`).

## For UW

- Ids must be derived from names at build time and frozen per table; an author never renumbers and a
  removal never shifts anything, so dead `_old` aliases and ID files disappear.
- Patching Native should be a first-class verb, not a file copy: "add option to menu X", "insert op
  before/after op Y in script Z", "replace block B", "disable entry E" — the compiler performs the
  anchored edit and errors when the anchor is gone.
- A feature is a unit with its own entries plus a declared order; the compiler computes the merge
  order, rejects duplicate ids and reports conflicts at build time (no `mods_active` list, no
  name-search helpers, no try/except glue).
- Removing or overriding Native content is a declaration, not a commented-out block, and the compiler
  keeps the disabled entry out of the generated tree.
- The compiler should record "was <Native entry>" provenance for every patched entry and be able to
  print the diff against the baseline, so the author's changes and Native's are never confused.
- Anything whose order is semantically meaningful (string tables, scene-prop indices, save-visible ids)
  should be hidden behind symbolic references that the compiler allocates and freezes.
- Engine limits (opcode set, 4-bit flag fields, save-visible ids) should surface as compiler
  diagnostics with a fix-it, not as comments in copied headers.
