# T-165 — How mods author big data (troops, items, parties, factions, scenes)

Mined 17 open-source Module System mods (`warband-opensource-mods/mods/`) for how `module_*.py` data
lists are written: hand-typed vs generated, helpers, copying, compact mass-authoring. 16 of the 17 mods
are cited below. `01-anno-domini-1257` ships only generated `ID_*.py` + scratch scripts, so it is cited
for id files and text-generating scripts only.
The plain kit `target/ref/kit/` is empty in this worktree, so "in the kit vs added" could not be checked;
every row below describes what the mod's own source does.

## Patterns

| # | pattern (generic, one line) | how it is done in MS (2-3 lines, operations named) | mods using it (count of 17 + names) | evidence (file:line, 1-3) |
|---|---|---|---|---|
| 1 | Flat hand-written list: every record typed out in full | One Python list of positional tuples, compiled in file order. No helper functions; text ids are string literals the compiler prefixes (`trp_`/`itm_`). Neighbour records share nothing. | most of 17; verified in 03, 04, 06, 12, 17 | `mods/06-the-last-days/ModuleSystem/module_party_templates.py:28`, `mods/04-medieval-conquests/src/module_parties.py:59`, `mods/12-beggars-diplomacy/source/module_troops.py:2650` |
| 2 | Stat bit-field helpers defined once above the list | `wp(x)` ORs the six `wp_*` proficiency bits; `wpe(m,a,c,t)` splits melee from ranged; `wpex(o,w,p,a,c,t)` gives one value per weapon class. Records then write `wp(130)` / `wpe(...)` in the proficiency field. | ≥4 of 17: 03, 11, 12, 14 | `mods/11-diplado/source/module_troops.py:53`, `mods/03-tainted-paths/src/module_troops.py:52`, `mods/12-beggars-diplomacy/source/module_troops.py:2652` |
| 3 | Optional trailing fields, decided by record length | Item field 9 (triggers) and field 10 (merchant factions) may be dropped; troop upgrade slots may be missing; a faction may carry its colour as field 6. Data code branches on `len(troop)` / `len(faction)`, exporters pad with `0` / `-1`. | ≥4 of 17: 06, 07, 14, 16 | `mods/16-warband-ai-chat/mod_src/module_items.py:23`, `mods/06-the-last-days/ModuleSystem/module_items.py:407`, `mods/07-brytenwalda/compiler/party_template.py:58` |
| 4 | One factory function per record family, called from the list | The function takes the varying part (banner code, skin, tier) and returns a complete record: id by string concatenation, flags/stats by bit-or, mesh and trigger inline. The list holds one call per member. | 3 of 17: 05, 14, 16 | `mods/05-persistent-world/module_items.py:74`, `mods/05-persistent-world/module_items.py:1075`, `mods/14-dickplomacy-reloaded/source/cstm_troop_trees.py:40` |
| 5 | Links declared after the list, resolved by id lookup | `upgrade(troops,"farmer","watchman")` / `upgrade2(...)` are plain calls below the closing bracket. The helper finds each id, pads the record to the 15-field layout, then writes the target index. | 3 of 17: 11, 12, 14 | `mods/12-beggars-diplomacy/source/header_troops.py:258`, `mods/12-beggars-diplomacy/source/module_troops.py:2660`, `mods/14-dickplomacy-reloaded/source/cstm_troop_trees.py:93` |
| 6 | Generator scripts that write source text instead of extending it | A stand-alone script imports the data lists, walks them, and appends formatted record lines to another module file (`open(...,"a")`) or writes a `.txt` of operations — a loop inside a module file cannot be reused by tools. | 3 of 17: 01, 05, 10 | `mods/10-persistent-kingdoms/generate_stockpiles_from_items.py:10`, `mods/10-persistent-kingdoms/generate_stockpiles_from_items.py:46`, `mods/01-anno-domini-1257/tom_name_change.py:7` |
| 7 | Sentinel pseudo-records used as anchors / end markers | A dummy record (`all_items_end`) or a blank troop is inserted so scripts and tools can bound a range or probe an insertion point; it costs a real id and must be skipped everywhere. | 2 of 17: 05, 14 | `mods/05-persistent-world/module_items.py:1393`, `mods/14-dickplomacy-reloaded/source/cstm_troops.py:66` |
| 8 | Component file merged into another mod's list by a named splice hook | The component ships its own list plus `modmerge(var_set)`: it pulls the host list out of a shared dict, finds the list's last line and calls `InsertBefore(pos, troops)`. | 2 of 17: 08, 17 | `mods/08-money-and-honour/src/KAOSPolitical_troops.py:21`, `mods/17-nmc-enhanced-module-system/util_wrappers.py:202` |
| 9 | Trigger blocks built by helper functions inside the record | `heraldic(...)` / `add_mesh(...)` return a whole `(ti_on_init_item, [operations])` tuple that the item record drops into field 9; ops are `store_trigger_param_*`, `call_script`, `cur_item_add_mesh`. | 2 of 17: 05, 16 | `mods/16-warband-ai-chat/mod_src/module_items.py:26`, `mods/16-warband-ai-chat/mod_src/module_items.py:552`, `mods/05-persistent-world/module_items.py:64` |
| 10 | Declarative spec constants + a generator class that expands them | A constants file lists the families (branches, tiers, level step); a class builds each troop from `(skin, branch, tier)` and inserts it; tiers are then wired with `upgrade2` by id. | 2 of 17: 02, 14 | `mods/14-dickplomacy-reloaded/source/cstmmerge_constants.py:21`, `mods/14-dickplomacy-reloaded/source/cstm_troop_trees.py:67`, `mods/02-twilight-of-the-sun-king/template_tools.py:33` |
| 11 | Record template with format placeholders, expanded into pages | A template object stores the skeleton with `{list_item}` / `{current_page}` fields; `generate_menus(input_list, pagelen=13)` slices the list, adds next/previous/back options and ids by `template_id + index`. | 1 of 17: 02 | `mods/02-twilight-of-the-sun-king/template_tools.py:74`, `mods/02-twilight-of-the-sun-king/module_game_menus.py:15126` |
| 12 | Ids treated as numbers (`base_id + n`) | Options compute an id with `(store_add, ":dest_npc", "trp_npc1", x)` inside a comprehension; helper classes (`add`, `sub`, `price`) defer the same arithmetic to export time. | 2 of 17: 05, 07 | `mods/07-brytenwalda/source/module_game_menus.py:19937`, `mods/07-brytenwalda/source/module_game_menus.py:19944`, `mods/05-persistent-world/header_lazy_evaluation.py:5` |
| 13 | Post-insert fix-up pass over positional references | After `troop_list.insert(insert_index, troop)` every later record's upgrade target is shifted (`troop[14] += troops_added`, guarded by `len(troop)>=15`), then the new upgrades are wired by id. | 1 of 17: 14 | `mods/14-dickplomacy-reloaded/source/cstm_troop_trees.py:82`, `mods/14-dickplomacy-reloaded/source/cstm_troops.py:52` |
| 14 | Metadata smuggled into an unused field, stripped by a post-pass | Fake trigger records starting with a float sentinel (`tag_item_class = -100.0`) carry category/value data in an item's trigger list; `pre_process_items()` at import collects them into side tables and pops them. | 1 of 17: 05 | `mods/05-persistent-world/module_items.py:52`, `mods/05-persistent-world/module_items.py:1439` |
| 15 | Compiler-level injection point instead of editing the list | The data list holds an `inject("items")` placeholder; plugins register elements under that name and `handle_list_injections` splices them in while loading. | 1 of 17: 09 | `mods/09-between-empires/SourceCode/compiler.py:1399`, `mods/09-between-empires/SourceCode/compile.py:172` |
| 16 | Alias a record to another record's id + modifier | A whole item is defined as a two-element reference `(itm_base, imod_cheap)` rather than a copied record; the pair resolves to the base id plus the modifier. | 1 of 17: 06 | `mods/06-the-last-days/ModuleSystem/module_troops.py:400` |
| 17 | Version / compile switch re-opening the list at the end | `if wb_compile_switch==1: items+=[ ... ]` after the closing bracket; extra records exist only for one build target, so ids differ between builds. | 1 of 17: 06 | `mods/06-the-last-days/ModuleSystem/module_items.py:1448`, `mods/06-the-last-days/ModuleSystem/module_troops.py:410` |
| 18 | One tuning constant wrapped per record by a helper | `scale = 1.5` plus `scale_f(coord)` wraps each coordinate; rows that forget the wrapper keep raw values, so one file mixes both systems. | 1 of 17: 04 | `mods/04-medieval-conquests/src/module_parties.py:46`, `mods/04-medieval-conquests/src/module_parties.py:86` |
| 19 | Compiled number turned back into names by scanning header dicts | `get_flags_from_bitmap(header_module,"sf_",bitmap)` walks a header module's `__dict__` and collects every constant set in the bitmap, so raw flags are re-rendered as readable source. | 1 of 17: 02 | `mods/02-twilight-of-the-sun-king/template_tools.py:112`, `mods/02-twilight-of-the-sun-king/module_game_menus.py:15123` |
| 20 | Copy-pasted filler records where a loop would do | Seven identical dummy troop rows differing only in the id string are typed by hand; nothing derives them, so one equipment change must be repeated seven times. | 1 of 17: 08 | `mods/08-money-and-honour/src/KAOSPolitical_troops.py:7` |

## Engine facts learned

- Troop records are positional; upgrade targets live in slots 14 and 15 and are often absent, so data code
  guards with `len(troop) >= 15` / `== 16`. (`mods/14-dickplomacy-reloaded/source/cstm_troops.py:54`,
  `mods/14-dickplomacy-reloaded/source/cstm_troop_trees.py:84`)
- The exporter pads: inventory to 64 slots with `-1 0`, party-template stacks to 6 with `-1`; a stack with
  more than 4 fields is accepted but the extras are dropped with a warning.
  (`mods/07-brytenwalda/compiler/troop.py:87`, `mods/07-brytenwalda/compiler/party_template.py:58`)
- The attribute word is four 8-bit values; starting level sits above them via `level_bits`/`level_mask`;
  proficiencies are 10-bit each; skills are emitted as 32-bit words; the face code is two 64-bit values
  written word-reversed. (`mods/07-brytenwalda/compiler/troop.py:91`, `mods/07-brytenwalda/compiler/troop.py:111`)
- Item flags and item type share one word: the type is the low byte, so tools mask it (`item[3] & 0xff`,
  `& ibf_armor_mask`) to branch on the record kind. (`mods/05-persistent-world/module_items.py:1420`,
  `mods/10-persistent-kingdoms/generate_stockpiles_from_items.py:12`)
- Item fields 9 and 10 are optional: a 7-field item with no trigger list is valid and appears in released
  data. (`mods/16-warband-ai-chat/mod_src/module_items.py:23`, `mods/06-the-last-days/ModuleSystem/module_items.py:407`)
- A party template holds at most 6 stacks — the record header comment states the limit.
  (`mods/06-the-last-days/ModuleSystem/module_party_templates.py:25`)
- Faction relations are stored symmetric: the faction exporter builds an NxN float matrix and writes each
  declared one-way relation into both cells. (`mods/14-dickplomacy-reloaded/source/process_factions.py:17`)
- Ids are file order; the compiler regenerates the `ID_<entity>.py` constant files from the exported
  (id, index) map, masking to 32 bits. (`mods/13-catplomacy/compile.py:28`, `mods/13-catplomacy/compile.py:426`,
  `mods/01-anno-domini-1257/ID_troops.py:1`)
- A menu has 16 option slots; a template that paginates must leave 3 for next/previous/back (13 usable).
  (`mods/02-twilight-of-the-sun-king/template_tools.py:41`)
- Build-time values exist: `LazyIdEvaluation` objects are resolved by the processor while it walks an id,
  so `price(itm_x)` is computed during export. (`mods/05-persistent-world/header_lazy_evaluation.py:5`,
  `mods/05-persistent-world/header_lazy_evaluation.py:32`)

## Pain points

- Inserting records anywhere but the tail silently breaks numeric upgrade links; the mod then re-walks and
  patches every later record by hand (`mods/14-dickplomacy-reloaded/source/cstm_troop_trees.py:82`).
- Upgrade trees are declared far from the troops, by string id, and a typo is a `print` during the build,
  not a failure (`mods/12-beggars-diplomacy/source/header_troops.py:261`).
- Short records are legal and mean different things at different lengths, so every consumer re-implements
  the padding rules (`mods/11-diplado/source/header_troops.py:267`).
- No record template or inheritance for data: dummy and filler rows are copy-pasted
  (`mods/08-money-and-honour/src/KAOSPolitical_troops.py:7`).
- Info the engine has no field for (categories, herd data) gets smuggled into an unused field and cleaned
  up at import (`mods/05-persistent-world/module_items.py:52`).
- Python loops cannot be "saved": tools that need the data write stand-alone scripts that textually append
  lines to module files, so the loop and its output drift apart
  (`mods/10-persistent-kingdoms/generate_stockpiles_from_items.py:46`).
- Merging two mods means splicing Python lists by line number; the component must hope the anchor still
  exists and catch `NameError` when an id is missing
  (`mods/08-money-and-honour/src/KAOSPolitical_troops.py:21`, `mods/14-dickplomacy-reloaded/source/cstm_troops.py:36`).
- Relations written once must be mirrored by a separate build pass, and a wrong name only prints
  (`mods/14-dickplomacy-reloaded/source/process_factions.py:27`).
- The reverse direction (compiled number → readable names) needs a helper that walks header module dicts
  (`mods/02-twilight-of-the-sun-king/template_tools.py:112`).

## For UW

- A record is named and can be derived: `base`/mixin plus field overrides replaces copy-paste and keeps
  every derived record visible to the compiler.
- Links between records are a verb declared on the record (`upgrade a, b, c`), resolved after all records
  exist, so declaration order and insertion never matter.
- Ids are the compiler's job: it assigns them, renumbers on insert, and keeps id arithmetic symbolic
  instead of a literal offset.
- Repetition is one closed form (`troops for faction in ... tiers 4..7`) that produces named records with
  derived stats — and it lives in the source, not in a side script that rewrites text.
- Optional data is a field, not a fake trigger: categories, tags and arrays-as-slots are first-class record
  members the compiler validates.
- Merging is a named operation (extend/override a list from another module) that fails to compile when the
  anchor is missing, instead of a line-number splice.
- Relations declared once are symmetric by default; the mirror is derived, not written.
- Bad data is a compile error: missing link targets, over-long stacks and out-of-range stats are rejected
  before the engine loads anything.
