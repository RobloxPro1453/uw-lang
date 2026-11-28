# UW core language v0 — specification

Design principle: **Module System = Assembly, UW = Python**. Engine words come from the kit, never
invented; every object can get fields; every UI part shares one property set.
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
item       := const | var | fn | on | while | every | ui
const      := "const" NAME "=" expr
var        := "var" target ("=" expr)? ("in" expr ".." expr)?
target     := NAME                      // feature variable (saved with the game)
            | KIND "." NAME             // field on every object of a kind: agent, troop, party, town,
                                        // castle, village, faction, item, player
fn         := "fn" NAME "(" (NAME ("," NAME)*)? ")" block
on         := "on" path ("(" NAME ")")? ("when" expr)? block      // event; optional bound object
while      := "while" "in" path block                              // continuous while a context is on
            | "while" expr block ("otherwise" block)?              // continuous state (in missions)
every      := "every" duration block                               // every 6 hours { … }
ui         := "ui" "." NAME NAME "{" (NAME "=" expr)* "}"          // ui.bar stamina_bar { ... }
block      := "{" stmt* "}"
stmt       := "let" NAME "=" expr
            | place ("=" | "+=" | "-=" | "*=" | "/=") expr
            | call
            | "if" expr block ("else" (block | if-stmt))?
            | "for" NAME "in" expr ("where" expr)? block
            | "return" expr?
            | "after" duration ("as" NAME)? block                   // once, that long from now
            | "cancel" NAME                                         // a pending `after … as NAME` won't run
duration   := NUMBER UNIT | NAME (a const holding one)              // 6 hours, 0.5 seconds
UNIT       := second(s) | minute(s) | hour(s) | day(s) | week(s)
place      := path                       // player.agent.speed_modifier, x, a.stamina
call       := path "(" (expr ("," expr)*)? ")"     // scene.town_6_center.jump(0), troop.player.add_gold(50)
expr       := or-expr with precedence: or < and < not < comparison (== != < <= > >=) < + - < * / < unary -
primary    := literal | path | call | "(" expr ")"
path       := NAME ("." NAME)*
```

## 4. Meaning (v0)

- Numbers are fixed-point: every UW number is stored ×1000 in the game (Native has integers only). The
  compiler preserves the ×1000 scale across `*`, `/`, `*=` and `/=` (`(a * b) / 1000`, `(a * 1000) / b`)
  and converts at the engine boundary (an engine op that takes an integer gets value/1000; a percent
  `150%` given to `agent.speed_modifier` becomes 150 because the op takes percent). Users never see this.
- User functions (`fn foo(a, b) { ... return x }`) lower to `script_uw_<feature>_<name>` and can be
  called within the same feature (`foo(1, 2)`) or across features (`other.foo(1, 2)`), both as statements
  and as value expressions (reading `reg0` into a fresh local; up to 15 parameters, the engine VM limit).
- `x += r per second` inside a continuous block (`while …`, `on mission.update`) adds `r × elapsed`
  where elapsed is the real time since the previous run (compiler-measured; `store_mission_timer_c_msec`,
  header_operations.py:1331). Outside a continuous block a rate is an error.
- `var … in lo..hi`: every write is clamped into the range.
- `var KIND.NAME = v`: a slot on that kind, number chosen by the compiler above every slot the kit uses for
  that kind; initialised to `v` when the object appears (agents: on spawn; others: at game start).
  The number is kept in the project's `slots.lock` and never changes (saved games store slots by number):
  new fields go above the kit's and the lock's numbers, a deleted field's number stays reserved (warning),
  and a kit that starts using a locked number is a build error §2).
- Feature variables: globals named `$uw_<feature>_<name>`.
- `on game.start` appends to Native's `game_start` after Native's own steps.
- `while in mission {}` and top-level `while cond {} otherwise {}` run every frame in every mission
  that has the player (compiler-generated dispatcher, §5). `otherwise` runs on every frame the
  condition is false.
- **Time** (D2). Seconds and minutes are *mission time* (real time spent in scenes, the saved clock
  `$uw_clock_ms`; it does not move on the campaign map). Hours, days and weeks are *game time* (campaign
  hours, `store_current_hours`; whole hours only, `every 1.5 hours` is an error).
  - `every D { … }` runs again and again; blocks of one clock and cadence are spread over the period
    (three `every 1 day` blocks first run at hours 0, 8, 16) so heavy work does not pile on one tick. The
    next run is one period after the actual run: a long pause never fires catch-up runs.
  - `after D (as NAME) { … }` (a statement) runs the block once, D from now; running the statement again
    restarts the wait. `cancel NAME` stops a pending one (same feature). The block runs later from the
    dispatcher, so it sees feature variables but not the `let`s around it. Not allowed inside `while`
    (it would restart every frame).
  - Reading the clock: `mission.time` is the mission clock in seconds (the one `every 0.5 seconds` runs on);
    game time is the kit's `game.current_hours`. A duration is a value of its clock: `2 seconds` = 2 (seconds),
    `1 day` = 24 (hours). A cooldown is the modder's code, not a word: `var agent.rested_at = 0`, `player.agent.rested_at = mission.time + 2 seconds`,
    `if mission.time >= player.agent.rested_at`. `var x cooldown` is an error that shows this; a std one
    written in UW comes with D9.
  - A rate (`15 per second`) is allowed only in a continuous block, where it means rate × frame time.
- **Engine words come from the kit**. Every
  property, query, condition and action the language knows about the engine is read from the generated
  easy vocabulary (`kit-meta/easy/ops.json`, `crates/uw_core/src/lower/kit.rs`): `agent.speed` (the
  agent's velocity), `agent.speed_modifier`, `agent.hit_points`, `agent.is_alive`, `mission.timer_c_msec`,
  parameterized getters (`agent.wielded_item(hand)`), fixed-point math (`destination_fixed_point`),
  multi-argument setters/actions, and parameterized/subjectless conditions — reaching **901 / 1,060 (85.0%)**
  of all kit operations; trailing optional `[params]` stay at the engine default. `kit-meta/core/properties.toml`
  only holds mechanism the kit cannot name (`player.agent`, `mission.time`, key edges, `player.name`); it
  never reuses a kit name or holds a number (enforced by `tests/kit_vocab.rs`). Invented/mechanic verbs
  (`teleport`, `disable(native.*)`, `cooldown`) are explicitly forbidden.
  Meanings such as "is moving" and every threshold are the modder's code:
  `while key.held(alt) and player.agent.speed > 0.5 { … }`.
- **Kit things** (D4b). A thing of the kit is written `<kind>.<kit name>`: `troop.player`, `party.town_6`
  (Praven), `scene.town_6_center`, `mission_template.town_center`, `menu.start_game_0`. The kind is the
  singular of the atlas table (`parties` → `party`), the name is the entry's name in the kit; the atlas must
  have it (no invented names). A thing takes only its own kind's words: `troop.player.name` is `troop.name`
  (never `player.*`, which is a multiplayer player's).
- **Kit actions** (D4b). A kit `method`/`function` is a statement: `scene.town_6_center.jump(0)`,
  `party.main_party.relocate_near_party(party.town_6, 1)`, `screen.mission()`. A parameter naming a thing
  takes its id as it is, a number goes in whole units, a text through a string register; an action has no
  value. A setter whose value is a thing takes the id as it is: `mission.jump = mission_template.town_center`.
- A kit constant is written by its easy name from `kit-meta/easy/constants.json` (`ca.strength` → `ca_strength`,
  `skill.*` are things: `skill.athletics`); it goes into an operation as it is. A kit parameter named `value`
  is the call's own argument (`troop.player.raise_attribute(ca.strength, 3)`).
- A text given to a kit operation goes in through one of the compiler's string registers (`s60..s67`,
  `str_store_string`), never as a bare quick string.
- `on menu.<name>.enter { … }` runs whenever the kit's menu opens, after the menu's own enter operations
  (appended to field 4 of the menu record). The engine opens `menu.start_game_0` right after `game.start`:
  that is where a mod skips character creation (`examples/alex/main.uw`).
- Numbers at the engine boundary: a kit value is whole units, so it is ×1000 when read and ÷1000 when
  written; an object (`{object}`) goes in as the engine's id, unscaled. `150%` is the number 150.
- A position value (`agent.speed`, `agent.position`) is read as `.x .y .z` (metres) or, used as a number,
  as its length in metres (`player.agent.speed > 0.5`, `player.agent.speed != 0`). The position registers are
  the compiler's (`pos56..pos63`), never written by the modder.
- A property with only a condition is a yes/no property: written bare it is a condition
  (`while player.agent.is_alive`), read as a value it is 1 or 0. A feature's own field or variable of the same
  name wins over it.
- `key.held(alt)`: true while the key is down (`key_is_down`, header_operations.py:70; key names from
  `key_*` constants, header_triggers.py, `key.left_alt` = 0x38; `alt` is an alias of `left_alt`).
- Errors are Turkish-friendly English messages with `file:line`, never a Python traceback.

## 5. Generated infrastructure (compiler, invisible to the modder)

1. Dispatchers (D2; `crates/uw_core/src/lower/time.rs`). The hidden trigger count never grows with the
   mod: game clock = ONE simple trigger every game hour calling `script_uw_game_clock` (reads the hour once,
   runs each due `every`/`after`); mission clock = in every mission template that has the player agent,
   ONE trigger at interval 0 calling `script_uw_mission_frame` + ONE `ti_before_mission_start` trigger
   (marks the first frame). The frame script measures the frame's ms (`store_mission_timer_c_msec`,
   capped at 1000 ms so a stall fires no burst), adds it to `$uw_clock_ms`, runs the due mission-time
   blocks, then each continuous block in order, each in its own `try_begin … try_end` (a failing
   condition never stops another feature). `every` keeps its next due time in `$uw_<feature>_every<k>`,
   `after` in `$uw_<feature>_after_<name>` / `_after<k>` (0 = nothing pending).
   `lower::time::report` prints each block's operation count and the busiest/quietest game hour and the
   usual/worst frame (static count: a loop counts once).
2. Agent init (D4a): one `ti_on_agent_spawn` trigger per mission template writes every agent field's start
   value (clamped) into the spawned agent. A start value on another kind's field is an error for now.
   Build (D4a, `crates/uw_cli/src/core_build.rs`): a folder whose `main.uw` has no `mod` line is a core
   project; the dispatcher triggers go to every mission template except `multiplayer_*`.
3. HUD (D3; `crates/uw_core/src/lower/ui.rs`): ONE presentation `prsnt_uw_hud`
   (`prsntf_read_only | prsntf_manual_end_only`, no mesh). Load: `set_fixed_point_multiplier 1000`, every
   `ui.*` part's overlays (ids in `$uw_hud_<part>_*`), `presentation_set_duration 999999`. Run: each part's
   bound values (`value`, `max`, `visible`) read again every frame. No trigger of its own: the mission
   dispatcher starts it whenever `neg|is_presentation_active` (so it also comes back after a full-screen
   window wiped it). Screen: x 0..1000, y 0..750 (top = 750); `mesh_white_plane` is scaled 50 per screen unit.
4. Start flow (D4b): no compiler word. The modder writes it with kit things and actions in
   `on menu.start_game_0.enter` (`mission.jump = …`, `scene.<x>.jump(0)`, `screen.mission()`); the build
   appends the block to the kit menu's enter operations (`core_build.rs`, field 4).

## 6. UI parts

Every part (`bar`, `text`, `image`, `button`, `panel`) takes the same properties, all optional with defaults:
`position` (`bottom_left`, `top_right`, … or `(x, y)` in screen units 0..1000), `size (w, h)`, `color`,
`alpha` (0..100%), `texture` (mesh name), `background`, `font_size`, `label`, `visible` (expr).
`bar` adds `value`, `max`. A `panel { … }` holds other parts.

Lowered in v0 (D3): `ui.bar` (`value` required; `max` default 100, `position` default bottom_left, `size`
default (200, 12), `color` default #3FA34D, `background` default black at half the alpha, `alpha`, `label`,
`visible`) and `ui.text` (`label` required, `position`, `color` default white, `alpha`, `font_size` as a
percent, `visible`). Anchors: bottom_left, bottom_center, bottom_right, center_left, center, center_right,
top_left, top_center, top_right, 20 units from the edges. Position, size, colors and label are fixed when
the HUD opens; `value`, `max` and `visible` follow their expressions every frame. A label with `{…}` values
(`"Stamina: {player.agent.stamina}"`) opens filled and is written again every frame (D5); image/button/panel
come with `screen` (D6).

Text with values (D5): in any text, `{expr}` shows a number as whole units (`75.6` shows `75`) through the
compiler's own registers `reg52..reg59` (Native never uses them; at most 8 numbers in one text), a text
constant inline, a text value by its string register. Registers never show in UW code.

## 7. Keys (D3)

`on key.pressed(q) { … }` runs on the frame the key goes down (the engine's edge, `key_clicked`);
`on key.released(q) { … }` on the frame it comes up (the dispatcher keeps last frame's state in
`$uw_<feature>_key<k>`). Both run inside the mission dispatcher, take `when <cond>`, add no trigger. The
name is a keyboard key when the kit has `key_<name>` (aliases of `key.held`: `alt` → `left_alt`), else a
game control the player can rebind (`gk_<name>`: `kick`, `jump`, …). In expressions: `key.held(q)`,
`key.pressed(q)`, `control.held(kick)`, `control.pressed(kick)`. The ops live in
`kit-meta/core/properties.toml`, not in the compiler.
