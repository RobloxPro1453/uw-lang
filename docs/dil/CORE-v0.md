# UW core language v0 — specification

Decision: docs/meclis/yon3/dil/KARAR-DIL.md (§2b flexibility rule: few general verbs; the target's type
decides the meaning; every object can get fields; every UI part has the same property set).
Crate: `crates/uw_core` (AST in `src/ast.rs` is the contract between parser and lowering).

## 1. Files

- `main.uw`: game-wide things (game start, player). `features/**/*.uw`: one feature per file, its name is
  the file stem (`features/sprint.uw` → feature `sprint`). Every file is active; order = `main.uw` first,
  then features sorted by path. Old-syntax files elsewhere keep compiling with the old front end.
- Names declared in a file live in that feature's namespace (`stamina.MAX` from another file); object
  fields (`var agent.stamina`) are global to the mod (one field per object kind and name; a second
  declaration of the same field is an error naming both places).

## 2. Lexical

- Comments `//` to end of line. Identifiers `[A-Za-z_][A-Za-z0-9_]*`. Keywords (fixed list):
  `const var let fn on while in otherwise if else for where return and or not true false per`.
- Literals: integer `12`, decimal `0.5`, percent `150%`, rate `15 per second` (also `per minute`,
  `per hour`), color `#3FA34D`, string `"..."` (with `{expr}` interpolation), tuple `(220, 14)`,
  range `0..100` (only after `in` in a var declaration).

## 3. Grammar (v0)

```
file       := item*
item       := const | var | fn | on | while | ui
const      := "const" NAME "=" expr
var        := "var" target ("=" expr)? ("in" expr ".." expr)?
target     := NAME                      // feature variable (saved with the game)
            | KIND "." NAME             // field on every object of a kind: agent, troop, party, town,
                                        // castle, village, faction, item, player
fn         := "fn" NAME "(" (NAME ("," NAME)*)? ")" block
on         := "on" path ("(" NAME ")")? ("when" expr)? block      // event; optional bound object
while      := "while" "in" path block                              // continuous while a context is on
            | "while" expr block ("otherwise" block)?              // continuous state (in missions)
ui         := "ui" "." NAME NAME "{" (NAME "=" expr)* "}"          // ui.bar stamina_bar { ... }
block      := "{" stmt* "}"
stmt       := "let" NAME "=" expr
            | place ("=" | "+=" | "-=" | "*=" | "/=") expr
            | call
            | "if" expr block ("else" (block | if-stmt))?
            | "for" NAME "in" expr ("where" expr)? block
            | "return" expr?
place      := path                       // player.agent.speed, x, a.stamina
call       := path "(" (expr ("," expr)*)? ")"     // player.teleport(town.praven.streets), disable(x)
expr       := or-expr with precedence: or < and < not < comparison (== != < <= > >=) < + - < * / < unary -
primary    := literal | path | call | "(" expr ")"
path       := NAME ("." NAME)*
```

## 4. Meaning (v0)

- Numbers are fixed-point: every UW number is stored ×1000 in the game (Native has integers only). The
  compiler converts at the engine boundary (an engine op that takes an integer gets value/1000; a percent
  `150%` given to `agent.speed` becomes 150 because the op takes percent). Users never see this.
- `x += r per second` inside a continuous block (`while …`, `on mission.update`) adds `r × elapsed`
  where elapsed is the real time since the previous run (compiler-measured; `store_mission_timer_c_msec`,
  header_operations.py:1331). Outside a continuous block a rate is an error.
- `var … in lo..hi`: every write is clamped into the range.
- `var KIND.NAME = v`: a slot on that kind, number chosen by the compiler above every slot the kit uses for
  that kind; initialised to `v` when the object appears (agents: on spawn; others: at game start).
  The number is kept in the project's `slots.lock` and never changes (saved games store slots by number):
  new fields go above the kit's and the lock's numbers, a deleted field's number stays reserved (warning),
  and a kit that starts using a locked number is a build error (meclis "maden" KARAR §2).
- Feature variables: globals named `$uw_<feature>_<name>`.
- `on game.start` appends to Native's `game_start` after Native's own steps.
- `while in mission {}` and top-level `while cond {} otherwise {}` run every frame in every mission
  that has the player (compiler-generated dispatcher, §5). `otherwise` runs on every frame the
  condition is false.
- General verbs (KARAR-DIL §2b): `teleport(target)`, `disable(x)`/`enable(x)`, `show`/`hide`,
  `spawn`/`remove`, `give`/`take`. Meaning by target type; each verb has a table "target type → ops" in
  `kit-meta/core/verbs.toml` (data, not compiler code).
- Properties (`player.agent.speed`, `player.name`) come from the generated easy vocabulary
  (`kit-meta/easy/*.json`, tasks T-130/T-131) plus `kit-meta/core/properties.toml` for the few
  hand-made ones (`player.agent` = the player's agent in the current mission, `player.face`).
- `key.held(alt)`: true while the key is down (`key_is_down`, header_operations.py:70; key names from
  `key_*` constants, header_triggers.py, `key.left_alt` = 0x38; `alt` is an alias of `left_alt`).
- Errors are Turkish-friendly English messages with `file:line`, never a Python traceback.

## 5. Generated infrastructure (compiler, invisible to the modder)

1. Mission dispatcher: in every mission template that has the player agent, one trigger at interval 0
   calling `script_uw_mission_frame`; that script measures elapsed ms and runs each feature's continuous
   blocks in order, each wrapped in its own `try_begin … try_end` (a failing condition never stops another
   feature).
2. Agent init: one `ti_on_agent_spawn` trigger per mission template initialising agent fields.
3. HUD manager: ONE presentation (`prsntf_read_only | prsntf_manual_end_only`) started after mission start;
   every `ui.*` declaration becomes overlays inside it, updated every frame from their bound expressions;
   restarted by a guard trigger if closed.
4. Start flow: `disable(native.character_creation)` + `player.teleport(<scene>)` from `game.start` become
   the menu/scene jump steps (`set_jump_mission`, `jump_to_scene`, `change_screen_mission`, mission
   template `town_center`; evidence in docs/meclis/yon3/dil/D2-MOTOR.md).

## 6. UI parts

Every part (`bar`, `text`, `image`, `button`, `panel`) takes the same properties, all optional with defaults:
`position` (`bottom_left`, `top_right`, … or `(x, y)` in screen units 0..1000), `size (w, h)`, `color`,
`alpha` (0..100%), `texture` (mesh name), `background`, `font_size`, `label`, `visible` (expr).
`bar` adds `value`, `max`. A `panel { … }` holds other parts.
