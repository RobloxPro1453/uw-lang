# T-158: How real mods organise `module_scripts.py`

Corpus: the 17 read-only mod copies under `warband-opensource-mods/mods/`. Script bodies are visible in
03, 04, 05, 06, 07, 08, 10, 14; 09, 11, 12, 13, 17 ship the generated `ID_scripts.py` and the call sites;
15 (a compiler copy) and 16 (callers only) ship no script definitions, so they are not cited. Every line
below was read in the mods themselves (the plain kit `target/ref/kit` is empty in this worktree).

## Patterns

| # | pattern (generic) | how it is done in MS | mods using it (of 17, named) | evidence (file:line) |
|---|---|---|---|---|
| 1 | Function value / callback: pass a script and call it later | Script ids are plain integers, so a name can travel in a local, a slot or a parameter and the callee runs `(call_script, ":var", args...)`; `-1` / 0 means "no callback". Used for per-object effect hooks, comparators and event dispatchers. | 13 - 01,04,05,06,07,08,10,11,12,13,14,16,17 | `mods/05-persistent-world/module_scripts.py:6452`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:494`, `mods/01-anno-domini-1257/module_mission_templates.py:2193` |
| 2 | Bare name + compiler-inserted `script_` prefix | A definition is `("<name>", [ops])`; every call site must spell `"script_<name>"`, and the build writes `script_<name> = <index>` into `ID_scripts.py`. The prefix is therefore a convention the compiler half-enforces. | 13 - 03,04,05,06,07,08,09,10,11,12,13,14,17 | `mods/05-persistent-world/module_scripts.py:20`, `mods/14-dickplomacy-reloaded/source/cstm_scripts.py:26`, `mods/09-between-empires/SourceCode/process_scripts.py:29` |
| 3 | Positional parameters, read at the top of the body | `(store_script_param, ":local", N)` (or the 1/2 shortcuts) copies arg N into a local; there is no name, type or default, and one parameter list serves every call site. Many scripts take 6-12 args. | 7 - 03,04,05,06,07,08,10 | `mods/03-tainted-paths/src/module_coop_scripts.py:28`, `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:21`, `mods/08-money-and-honour/src/yael_eqmgr_scripts.py:797` |
| 4 | Documented contract in a comment block | A `# script_<name>` header states Author/Called/Input/Output above the entry, because nothing in the syntax carries it; readers (and the next modder) treat it as the signature. | 6 - 03,04,06,07,10,14 | `mods/04-medieval-conquests/src/autoresolve.py:318`, `mods/07-brytenwalda/source/economy/__init__.py:13`, `mods/10-persistent-kingdoms/module_scripts.py:7270` |
| 5 | `cf_` prefix = predicate that is allowed to fail | A script whose first operations can fail returns "false" by aborting, so callers use it directly as a condition inside `try_begin`. The compiler warns when a script that is *not* named `cf_*` can fail at its top level. | 8 - 05,06,07,08,10,12,14,17 | `mods/07-brytenwalda/source/caravans/trade.py:82`, `mods/06-the-last-days/ModuleSystem/module_scripts_form.py:5879`, `mods/12-beggars-diplomacy/source/ID_scripts.py:91` |
| 6 | Single return value in `reg0` | The callee assigns `reg0` and the caller copies it out immediately; a function returning several values just uses `reg0`, `reg1`, ... and documents which is which. | 6 - 04,05,06,07,10,14 | `mods/05-persistent-world/module_scripts.py:1725`, `mods/04-medieval-conquests/src/autoresolve.py:323`, `mods/07-brytenwalda/source/economy/__init__.py:157` |
| 7 | Extra outputs through `s0..sN`, name says `_to_sN` | Text results are written into a string register and the name encodes the channel (`..._to_s0`, `..._to_s24`), so call sites know where the string lands without reading the body. | 4 - 06,07,11,14 | `mods/14-dickplomacy-reloaded/source/cstm_scripts.py:126`, `mods/06-the-last-days/ModuleSystem/ID/ID_scripts_mb.py:10`, `mods/07-brytenwalda/source/caravans/trade.py:265` |
| 8 | Engine hook scripts written as `game_*` stubs | The engine calls scripts by fixed names (`game_get_use_string`, `game_event_*`, `game_get_troop_wage`); mods declare empty `[]` bodies to keep the table complete and fill only the hooks they need. | 4 - 05,09,10,12 | `mods/10-persistent-kingdoms/module_scripts.py:912`, `mods/05-persistent-world/module_scripts.py:27`, `mods/09-between-empires/SourceCode/ID_scripts.py:2` |
| 9 | Hook/trigger result: `reg0` plus `set_trigger_result` | A hook writes the value the engine expects into `reg0` and then `(set_trigger_result, 1)`; `(set_trigger_result, 0)` is how a script declines an option/hit it was asked about. | 2 - 05,10 | `mods/05-persistent-world/module_scripts.py:1386`, `mods/10-persistent-kingdoms/module_scripts.py:2232`, `mods/05-persistent-world/module_scripts.py:6455` |
| 10 | Replacing native behaviour: rename, delegate, extend | The original native script is renamed (`orig_...`) and a new script with the native name calls it plus the mod's extra work; alternatively the mod appends a case straight into the native dispatcher. | 3 - 06,07,17 | `mods/06-the-last-days/ModuleSystem/module_scripts_form.py:5812`, `mods/07-brytenwalda/source/multiplayer/scripts.py:2660`, `mods/17-nmc-enhanced-module-system/modmerger_readme.txt:99` |
| 11 | One list per feature file, concatenated in Python | Each module/package exports `scripts = [...]` (or `ai_scripts`, `common_warp_scripts`) and a parent file joins them with `+` / `.extend(...)`; the build sees a single flat list and keeps a changelog comment of added/removed ids. | 6 - 05,06,07,08,14,17 | `mods/07-brytenwalda/source/constable/__init__.py:149`, `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:17`, `mods/05-persistent-world/module_scripts.py:3874` |
| 12 | Generate the operation block with Python at import time | A plain Python function appends tuples to a list and the result is inserted as a script body, so repetitive blocks are produced by a `for` loop instead of being written out. | 3 - 01,04,05 | `mods/04-medieval-conquests/src/autoresolve.py:246`, `mods/05-persistent-world/module_scripts.py:3849`, `mods/01-anno-domini-1257/autoresolve.py:8` |
| 13 | A hand-written "standard library" of scripts | Mods ship a generic library (arrays, sorting, string helpers) with real doc blocks, guards and version stamps, and every other feature calls it as if it were built in. | 3 - 06,08,14 | `mods/14-dickplomacy-reloaded/source/array_scripts.py:33`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:33`, `mods/08-money-and-honour/src/yael_util.py:36` |
| 14 | Namespace prefix per author/mod in the id | Ids carry a short owner tag (`kt_`, `cstm_`, `warp_`, `dplmc_`, `coop_`) so copied-in code stays attributable and cannot collide with native names. | 5 - 03,04,06,11,14 | `mods/04-medieval-conquests/src/autoresolve.py:8`, `mods/14-dickplomacy-reloaded/source/cstm_scripts.py:126`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:20` |
| 15 | Big script split into `_aux` helpers | Long algorithms are cut into a public entry plus `*_aux` scripts that pass state through parameters and `reg0`; wrappers and call sites then name the helper `..._aux`. | 4 - 01,04,06,07 (+ `_aux` ids in 11) | `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:476`, `mods/11-diplado/source/ID_scripts.py:225`, `mods/07-brytenwalda/source/game_start/game_start_script.py:221` |
| 16 | Python conditional selects which operation block is compiled | A `(is_a_wb_script==1 and [...] or [])` expression splices one of two op-blocks into the same script, so one source ships both M&B and Warband behaviour. | 1 - 06 | `mods/06-the-last-days/ModuleSystem/module_scripts_form.py:5824`, `mods/06-the-last-days/ModuleSystem/module_scripts_form.py:5828` |
| 17 | Save and restore the shared registers by hand | Around any call the mod copies `reg0`/`reg1` into locals first and restores them afterwards; debug helpers bank their own register window because no register is private. | 3 - 05,06,10 | `mods/10-persistent-kingdoms/module_scripts.py:467`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:50`, `mods/05-persistent-world/header_debug.py:9` |
| 18 | Recursion and bounded `try_for_*` loops instead of control flow | Iteration is `try_for_range`/`try_for_parties` with end constants; recursion is a script calling itself (through a name, or a passed comparator) one round at a time. | 2 - 06,14 | `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:475`, `mods/14-dickplomacy-reloaded/source/array_scripts.py:65`, `mods/07-brytenwalda/source/economy/__init__.py:151` |

## Engine facts learned

- Scripts are stored by position, not by name: `scripts.txt` gets `<id> -1` (or `<id> <float>` for the
  special form) followed by the operation block, and `ID_scripts.py` is written from the same index, so
  inserting a script renumbers every script after it. Evidence: `mods/09-between-empires/SourceCode/process_scripts.py:18`, `:21`, `:29`.
- A script body must literally be a list of operations; a string or dict is a build error.
  Evidence: `mods/07-brytenwalda/compiler/script.py:13`.
- One operation takes at most 16 parameters; the compiler refuses more. This is the real ceiling on how
  many values a script signature can carry. Evidence: `mods/05-persistent-world/process_operations.py:191`.
- A "failing" operation ends the script silently. That is why predicates are named `cf_` and the compiler
  warns about it ("... can fail but it's name does not start with \"cf_\"", and the older copy's
  "Script can fail at operation #N. Use cf_ at the beginning of its name").
  Evidence: `mods/09-between-empires/SourceCode/compiler.py:1535`, `mods/01-anno-domini-1257/process_operations.py:448`,
  `mods/05-persistent-world/process_operations.py:222`.
- Locals are mapped per operation block with a hard limit of 128, using a local before assigning it is an
  error, and an unread local is a warning unless its name starts with `unused`.
  Evidence: `mods/05-persistent-world/process_operations.py:17`, `:26`, `:35`, `:39`.
- `reg0`.. are shared globals with the caller: any call overwrites them, so a callee that wants to use `reg0`
  as scratch must save the incoming value first (`"#reg0's value should be saved ..."`).
  Evidence: `mods/10-persistent-kingdoms/module_scripts.py:467`.
- There is no heap: the standard trick is a disabled dummy party whose slots are the elements, slot 0 holding
  the length; a magic number/version slot makes the handle type-checkable and save-compatible.
  Evidence: `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:34`, `mods/14-dickplomacy-reloaded/source/array_scripts.py:23`.
- The engine exposes a sort that takes a comparator script id (`array_sort_custom`), yet no mod in the corpus
  uses it; they all write their own mergesort in scripts.
  Evidence: `mods/03-tainted-paths/src/header_operations.py:3327`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:475`.
- The header declares multi-value returns (`return_values`, `store_num_return_values`, `store_return_value`,
  opcodes 3015-3017) but no mod in the corpus uses them; every hand-written return still goes through
  `reg0`/`sN`. Evidence: `mods/03-tainted-paths/src/header_operations.py:3036`.
- Hook scripts must exist under their exact engine name; unused ones are kept as `("game_...", [])`, so the
  engine-visible surface of a mod is a name list that builders patch rather than a call graph.
  Evidence: `mods/10-persistent-kingdoms/module_scripts.py:912`.
- Build-time checking exists (parameter count, unused variable, missing block) but runtime has none: no
  exception object, no error code convention beyond `-1` sentinels and `set_trigger_result, 0`.
  Evidence: `mods/05-persistent-world/process_operations.py:191`, `mods/05-persistent-world/module_scripts.py:6455`.

## Pain points

- The signature exists only in a comment; when code changes the comment is not updated, so a script can
  document `arg1 = agent_no` while param 1 is really `pos_no` and nobody notices (`mods/03-tainted-paths/src/module_coop_scripts.py:3448` vs `:3453`).
- Every argument has to be re-copied into a local at the top of every body, and every call has to repeat the
  argument count by hand; the 12-parameter generic hooks show how quickly that gets unreadable
  (`mods/05-persistent-world/module_scripts.py:6404`).
- Only one shared return register: multi-value results turn into conventions in the name and in the doc block
  (`reg1`, `s1`, `_to_s24`), plus "copy `reg0` right now" discipline at call sites
  (`mods/05-persistent-world/module_scripts.py:6423`).
- Register scarcity: `reg0` is both the return channel and the favourite scratch, so mods write save/restore
  pairs and one mod even writes a static register allocator for debug output
  (`mods/05-persistent-world/header_debug.py:9`).
- No data structures: arrays, stacks and sorting are re-implemented per mod on top of parties and slots
  (`mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:33`, `mods/14-dickplomacy-reloaded/source/array_scripts.py:33`).
- Error handling is split between "abort silently" (`cf_`), sentinel values and build-time asserts; a mod
  cannot report a soft error to its caller except by inventing another return convention.
- Extending native logic means editing a huge native script by hand (one mod's network dispatcher is a
  hand-appended case list, `mods/07-brytenwalda/source/multiplayer/scripts.py:2660`) or shipping
  rename-and-delegate wrappers and a build-time merger (`mods/17-nmc-enhanced-module-system/util_scripts.py:10`).
- Organisation has no middle ground: either one 13k-15k line file
  (`mods/05-persistent-world/module_scripts.py:1`, `mods/10-persistent-kingdoms/module_scripts.py:15000`) or
  dozens of packages that must be concatenated by hand in the right order
  (`mods/07-brytenwalda/source/constable/__init__.py:149`).
- Duplicated boilerplate is fought with Python: `store_script_param` runs, debug prints, macro-like helpers
  (`mods/08-money-and-honour/src/yael_util.py:60`) and generated bodies (`mods/05-persistent-world/module_scripts.py:3849`) all exist
  only because the language has no syntax for them.

## For UW

- Make parameters named, typed and defaulted, with the signature written once and checked at the call site
  instead of in a comment.
- Return tuples/records natively; never expose a shared global return register, and never require the caller
  to copy a value before the next call.
- Offer `cf_`-style fallible predicates as a first-class `if` / `try` condition; a failing operation should be
  a statement the compiler understands, not an implicit abort.
- Provide the callback form as a general verb (pass a function, call it later, "none" as a value) instead of
  teaching modders to smuggle script ids in variables.
- Ship the library the mods keep rewriting: lists, maps, sort with a comparator, string building, bounds
  checks - generated to slots/parties under the hood.
- Generate the engine-facing plumbing automatically: hook stubs, the id table, feature-file aggregation, and
  the append/rename/replace merge that modder tooling does at build time.
- Keep the low-level escape hatch (registers, raw operation blocks) available and visible in generated
  output, so an expert can still inspect and override what the compiler emitted.
