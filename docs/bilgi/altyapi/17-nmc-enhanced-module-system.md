# Infrastructure scan — 17-nmc-enhanced-module-system (T-154)

What it is: not a single game mod but a **mod compilation that ships its own forked Module System** (Warband
1.153 kit, Python 2) plus a Python-side merge framework; nine merged components (freelancer, formations, formAI,
PBOD, GPU widget kit, MCC, tournament, trees, health regen) are carried as `<modid>_<component>.py` files. The
merged `ID files/ID_scripts.py` declares 956 scripts and `ID files/ID_presentations.py` 89 presentations, i.e. the
source mostly consists of script/trigger/presentation Python that the compiler and engine force into low-level form.
Paths below are relative to `mods/17-nmc-enhanced-module-system/`.

| # | category | pattern (generic, one line) | evidence (file:line) | approx lines | times reused |
|---|----------|-----------------------------|----------------------|--------------|--------------|
| 1 | BUILD | Composition by plugin files: each mod is a set of `<mod>_<component>.py` units whose `modmerge_<component>()` is discovered and called by the build, so no unit edits another unit's source | `modmerger.py:88-97`, `modmerger_options.py:21-40`, `module_constants.py:2416-2431` | ~570 | 9 mods × ~10 components |
| 2 | BUILD | Branded, idempotent source-hook installer: inserts a marked block with a version+type bitmask into an existing file, keeps `.bak` copies, can uninstall | `modmerger_installer.py:72-113`, `modmerger_installer.py:344-362` | ~440 | 27 module files |
| 3 | BUILD | Object wrappers over the module data lists: find a trigger/menu/prestation by id, then search, insert, replace or delete single op lines inside its op block | `util_wrappers.py:53-77`, `util_wrappers.py:79-121`, `util_wrappers.py:349-385` | ~650 | dozens of hooks |
| 4 | BUILD | Directive list language for build-time script surgery (remove/rename/add/insert/remove-range, add op lines at position flags) | `util_scripts.py:10-19`, `util_scripts.py:28` | ~225 | 8 directive kinds |
| 5 | BUILD | Source reformatter for the block language: re-indents `try_*`/`try_end` nesting before compiling | `process files/process_line_correction.py:7-30` | ~33 | every build |
| 6 | STATE | Per-object state in numbered object slots, with hand-written allocation comments naming the Native slot each number shadows and the capacity left | `pbod_constants.py:14-16`, `pbod_constants.py:19-39`, `module_constants.py:32-33` | ~210 | ~60 new slots |
| 7 | COLLECTION | Dynamic arrays as a script library (create/size/get/set/push/pop/insert/remove) over hidden `{!}` dummy troops used as array heaps/records | `ID files/ID_scripts.py:732-756`, `module_troops.py:2091-2093`, `module_game_menus.py:627-629` | ~25 scripts | 20+ call sites |
| 8 | COLLECTION | Array/2-D matrix emulated as a `..._begin` slot base plus an index computed at use site, no bounds check, no type | `pbod_constants.py:36-38`, `pbod_constants.py:60-64`, `freelancer_scripts.py:504-528` | ~60 | ~40 |
| 9 | CONFIG | Settings stored in object slots inside a hand-reserved number range, lazily defaulted on first read, edited by a dedicated options screen | `pbod_presentations.py:2891-2895`, `pbod_constants.py:68-69` | ~120 | 15+ options |
| 10 | UI | Options form built by hand from overlays: scroll container, per-row y cursor arithmetic, mesh rows, checkbox/number/combo widgets | `pbod_presentations.py:2885-2950`, `pbod_presentations.py:3010-3019` | ~700 | 1 per component |
| 11 | UI | Widget→setting binding by a long if-chain on the clicked overlay id, writing the value straight into a slot | `pbod_presentations.py:3256-3330` | ~75 | 15+ widgets |
| 12 | UI | Read-only report/HUD screen: title + one overlay per line, each line positioned from a running y counter | `freelancer_presentations.py:26`, `freelancer_presentations.py:40-57` | ~250 | 20+ screens |
| 13 | UI | Reusable widget toolkit: widget handles and values live in two dummy-troop slot arrays, widgets addressed by index constants | `gpu_scripts.py:77-101`, `gpu_constants.py:45-47` | ~740 | one kit, many screens |
| 14 | UI | Scrollable list + combo filter + detail pane report, with paging and per-row overlay bookkeeping | `module_presentations.py:18539-18564`, `module_presentations.py:18568-18594` | ~500 | several reports |
| 15 | TEXT | String assembly by writing registers/`str_*` into s0..s9 and embedding `{s19}`/`{reg0}` placeholders in the shown template | `freelancer_presentations.py:45-54`, `pbod_scripts.py:118-159` | ~1500 | everywhere |
| 16 | INPUT | Key bindings kept in global variables and used indirectly, so a screen can remap them at run time | `pbod_constants.py:132-133`, `pbod_constants.py:165-186`, `pbod_mission_templates.py:35` | ~120 | ~25 keys |
| 17 | INPUT | Key-rebinding screen: capture next key down, build a preview overlay, reject duplicate bindings, persist | `pbod_presentations.py:3442`, `pbod_presentations.py:3565`, `pbod_presentations.py:3635` | ~250 | 1 screen, 25 keys |
| 18 | INPUT | Distinguishing "key tapped" from "key held" to drive an order interaction, with the pending order remembered in a slot | `formations_mission_templates.py:109`, `formations_mission_templates.py:153`, `pbod_mission_templates.py:577` | ~120 | ~10 |
| 19 | TIMER | Own clock/cooldown because the trigger's own delay does not work: store the mission time of the press, compare with a fixed threshold | `formations_mission_templates.py:99-104`, `formAI_mission_templates.py:48-52` | ~40 | ~15 |
| 20 | EVENT | Turning a one-shot trigger into a re-armed condition trigger, and re-arming it manually, to work around lost once-flags | `pbod_mission_templates.py:296-303`, `pbod_mission_templates.py:381` | ~60 | several |
| 21 | AI | Battle-phase state machine over global variables, stepped by a short-interval trigger and read by every AI decision | `formAI_mission_templates.py:18-19`, `formAI_mission_templates.py:56-60`, `formAI_scripts.py:303-312` | ~1440 | 1 per battle |
| 22 | MATH | Hand-rolled geometry for formations (sqrt-based width, point-on-line, centering, direction vectors) | `formations_scripts.py:86`, `formations_scripts.py:1650` | ~300 | many |
| 23 | DEBUG | Per-module compile-time debug level constants, plus logging levels inside the Python build, no run-time introspection | `gpu_constants.py:9`, `health_regen_constants.py:25`, `util_common.py:4-27`, `modmerger_options.py:5-8` | ~80 | every component |

## Top 5 things a language should give built in

1. First-class dynamic lists/maps and strings with interpolation, so arrays are not slot-offset arithmetic on fake objects and text is not s-register juggling.
2. A declarative UI layer with layout, lists, forms and two-way binding to settings, so screens are not hand-positioned overlay coordinates plus id-comparison chains.
3. A real event model: working timers/cooldowns, reusable and re-armable triggers, and keys as remappable first-class values rather than fixed game keys.
4. Namespaced per-object state with compiler-checked allocation and automatic save migration, instead of manually numbered slots that shadow Native ones.
5. A module/plugin system with import, override and wrap of someone else's menus, triggers and presentations, plus static analysis that errors (scopes, limits) instead of printing warnings.

## Language/compiler features added on top of the plain kit

- `@"text"` literal anywhere in a statement: automatically allocated quick string, no `module_strings.py` entry (`process files/process_operations.py:327-361`, `:376-378`).
- Can-fail analysis: an op from `can_fail_operations` inside a script must be named `cf_*`, checked at compile time (`process files/process_operations.py:314-318`, `:444-449`; `header files/header_operations.py:1629`).
- Implicit global-variable declaration from left-hand-side ops (`lhs_operations` / `global_lhs_operations`), with a warning on read-before-assign (`process files/process_operations.py:403-414`; `header files/header_operations.py:1318`, `:1618`).
- Variable hygiene lints: same name used as both `$global` and `:local` warns; unassigned local is an error; unused locals, `store_script_param` misuse and a >128-local limit are reported (`process files/process_operations.py:250-253`, `:290-297`, `:440-460`).
- `try_*`/`try_end`-aware automatic indenting of `module_scripts.py` as a pre-pass (`process files/process_line_correction.py:13-14`).
- A build-time directive language for existing scripts: `SD_REMOVE`, `SD_RENAME`, `SD_ADD`, `SD_REPLACE`, `SD_INSERT_*`, `SD_REMOVE_RANGE`, `SD_OP_BLOCK_*` (`util_scripts.py:10-19`).
- A versioned plugin protocol: components declare `modmerge(varset|list)` and a branding block records merger version and component type bits, which the installer uses to generate the right call (`modmerger_installer.py:72-113`, `modmerger_header.py:4`, `modmerger_header.py:13-19`).
- Merge order and per-component enable switches are data, not code (`modmerger_options.py:21-40`).
- Build log verbosity as a single option (`modmerger_options.py:5-8`), and `process_scripts_show_script_name` to trace which source line emitted an op (`modmerger_options.py:16-19`).

## Native code the mod changed

- 27 Native `module_*.py` files carry an installed `# modmerger_start version=201 type=N` hook (`module_constants.py:2416`, `module_game_menus.py:19660`, `module_mission_templates.py:16476`, `module_variables.py:2`).
- Native `battle` presentation: the component finds the Native trigger and splices its code block into it (`formations_presentations.py:142`).
- Native mission templates: components append their trigger lists to existing missions (`pbod_mission_templates.py:2109-2116`, `formAI_mission_templates.py:151`).
- Native menus: components add their options into existing menus instead of creating new ones (`pbod_game_menus.py:188-192`).
- Report entries added into the Native camp/character menus (`module_game_menus.py:601-644`); Native lists (`module_presentations.py`, `module_simple_triggers.py`, `module_strings.py`, `module_troops.py`) are edited in place.
- The Module System itself is forked, not extended: `header files/`, `process files/`, `ID files/` are a modified 1.153 kit (`modmerger_options.py:13`, `modmerger_header.py:6`).
