# Changelog

## 0.1.0 - Initial public release

First public version of UW, a general-purpose modding language and compiler for Mount & Blade: Warband 1.174 that compiles through the official Module System 1.171.

### Language

- Constants, saved variables, per-object fields with value ranges (`var agent.stamina = 100 in 0..100`) and local `let` bindings.
- Functions (`fn`) callable within a feature or across feature files, with up to 15 parameters.
- Continuous blocks (`while ... otherwise`), scheduled blocks (`every`, `after ... as`, `cancel`) and engine events (`on game.start`, `on menu.<name>.enter`, `on key.pressed(...)`).
- Literals with units: percentages, colors, durations and rates (`15 per second`); fixed-point scaling is handled by the compiler.
- HUD parts (`ui.bar`, `ui.text`) collected into one shared presentation.
- Engine vocabulary generated from the kit: readable names for all 1,060 operations and for constants and records; 901 of the 1,060 operations are reachable from the core language.

### Compiler and tools

- `uw build` checks a project, compiles it with the Module System in an isolated staging copy, prints a static operation-cost report and installs the mod.
- Object-field slot numbers are locked in `slots.lock` so saved games stay valid.
- Time-based blocks of all features are merged into one campaign-clock trigger and one mission-frame trigger.
- `uw check`, `uw words`, `uw explain`, `uw browse`, `uw find`, `uw graph`, `uw rename`, `uw coverage`, `uw view`, `uw create`.
- A decompiler that renders Native's operation blocks and records as UW text.
- VS Code extension with highlighting, diagnostics, completion and hover.
- Examples: `alex`, `steve`, `praven_start`, `menu_demo`.
