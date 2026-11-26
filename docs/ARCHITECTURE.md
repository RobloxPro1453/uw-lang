# Architecture

## 1. Principle: the Module System is assembly, UW is Python

UW is a general-purpose, high-level language targeting the Warband 1.174 virtual machine. It is neither a small patching tool nor a library of ready-made mechanics.

- **No built-in mechanics.** The compiler knows only general language constructs (`const`, `var`, `let`, `fn`, `if` / `else`, `for`, `while ... otherwise`, `every`, `after`, `on`, `ui.*`). Gameplay mechanics are user code in `features/*.uw`.
- **Words come from the game.** Engine objects (`agent`, `troop`, `party`, `faction`, `scene`, `menu`, `item`, `quest`, ...), their properties and methods, and the engine constants are generated from the Module System 1.171 kit (`kit-meta/easy/ops.json`, `kit-meta/easy/constants.json`, `kit-meta/records/*.toml`), never invented.
- **No ceiling.** The goal is access to all 1,060 operations, the trigger and `game_*` hooks, and the 28 tables without leaving the language. Assembly-level burdens are the compiler's job: register conflicts, fixed-point scaling, `slots.lock`, `try_begin` / `neg|` / `this_or_next|` flags, a single frame/clock dispatcher and a single HUD presentation.

## 2. Build pipeline

```
 main.uw + features/*.uw (+ slots.lock)
      |
      v  uw_core   parse -> lower -> schedulers / HUD / fixed-point math / slot lock
 Lowered (scripts, triggers, presentations, menu and script additions, records)
      |
      v  uw_cli::core_build -> uw_check   verify every name against the atlas, suggest close names
 Checked (verified module patches and new records)
      |
      v  uw_emit   wrappers for changed modules + uw_runtime.py + ID_*.py pre-writes
 target/uw-build/<mod>/staging/   an isolated copy of the Module System
      |
      v  uw_kitrun   runs the kit's process_*.py steps with Python 2.7 (-B -E -s)
 Compiled .txt files
      |
      v  uw_cli::build   installs only into a folder carrying UW's own .uw-installed marker
 <Warband>/Modules/<mod>/
```

## 3. Crates

| Crate | Role |
|---|---|
| `uw_core` | Main language front end (`CORE-v0`): block-syntax parser, lowering (`lower/`), kit vocabulary binding (`lower/kit.rs`), schedulers (`lower/time.rs`), HUD compiler (`lower/ui.rs`), slot lock (`slots.rs`) and the Native-as-UW viewer (`view.rs`). |
| `uw_easy` | Generates the readable vocabulary for kit operations and constants (`kit-meta/easy/`). |
| `uw_atlas` | Index of the kit's identifiers, operations, constants and tables (`atlas.json` reader). |
| `uw_check` | Name, type and reference checker; produces the `Checked` patch and record representation. |
| `uw_emit` | Emits the Python 2.7 staging wrappers (`module_*.py`), `uw_runtime.py` helpers and `ID_*.py` pre-writes. |
| `uw_kitrun` | Builds a clean staging copy of the kit on every build and runs its `process_*.py` scripts; the kit folder is never written. |
| `uw_decomp` | Decompiler turning Native's operation blocks and table records into readable UW (`uw roundtrip`, `uw create`, `uw view`). |
| `uw_start` | Dry-run analysis of Native's `script_game_start` assignments (`uw explain`). |
| `uw_diag` | Source spans, line/column calculation and "did you mean ...?" diagnostics. |
| `uw_cli` | The `uw` command-line tool. |
| `uw_syntax`, `uw_lower` | Older record-oriented front end (files starting with `mod <name>`), kept for compatibility and golden tests. |

## 4. Design rules

1. **The kit is read-only.** The Module System folder is never written; builds run in a fresh staging copy.
2. **Native plus difference, deterministic.** Records the modder did not touch stay byte-identical to Native. The same input yields the same output on every machine.
3. **Save-game safety.** Slot numbers for object fields (for example `var agent.stamina`) are locked in `slots.lock`. The file only grows; the number of a deleted field is not reused.
4. **One dispatcher, one HUD.** However many feature files exist, the compiler emits at most one campaign-clock trigger (`script_uw_game_clock`), one frame trigger and one start trigger per mission (`script_uw_mission_frame`), and one shared HUD presentation (`prsnt_uw_hud`). Work with the same period is spread across time slices, and the per-frame operation cost is reported.
5. **Transparent fixed point.** All numbers in the language are thousandths-fixed-point (`0.5` is `500`). The compiler converts scale at engine boundaries (using the parameter information in `ops.json`) and preserves it across multiplication and division.
6. **Safe installation.** `uw build` overwrites only a mod folder carrying its own `.uw-installed` marker, never a Warband mod the user installed by hand.
