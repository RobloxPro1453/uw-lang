# UW language guide

This guide describes what works today. The formal definition is [dil/CORE-v0.md](dil/CORE-v0.md).

## 1. What UW is

Warband's official Module System 1.171 works like assembly: even dividing a number or drawing a small bar requires numbered registers (`reg0`, `s1`, `pos1`), hand-maintained slot numbers, fixed-point multipliers (`x1000`) and low-level tuples.

UW plays the role of a high-level language on top of it:

- It has no built-in sprint, stealth or stamina feature. It provides general constructs (`var`, `let`, `const`, `fn`, `if` / `else`, `for`, `while`, `every`, `after`, `on`, `ui.*`) and the engine's own vocabulary (`agent`, `troop`, `party`, `scene`, `menu`, `faction`, ...).
- Any mechanic is written by the modder in a few dozen lines. The compiler handles slot numbers (`slots.lock`), merges timed blocks into single triggers, and builds and installs the result.

## 2. Project structure

A UW project is a plain folder:

```
my_mod/
  main.uw            game-wide flow and start rules
  slots.lock         slot numbers kept by the compiler (keeps saved games valid; do not delete)
  features/
    stamina.uw       one feature per file, each with its own namespace
    sprint.uw
```

Build and install:

```
uw build my_mod
```

`uw build my_mod --no-install` compiles without writing into the Warband folder.

## 3. Language basics

### 3.1 Variables and object fields

```uw
const MAX_STAMINA = 100

// Saved variable (lives in the save game)
var counter = 0

// A new field on every agent; writes are clamped to 0..100
var agent.stamina = 100 in 0..MAX_STAMINA
```

- `let x = 10` is a temporary that lives only in its `{ ... }` block.
- Literals: decimals (`0.5`), percentages (`150%`), colors (`#3FA34D`), durations (`2 seconds`, `6 hours`, `3 days`) and rates (`15 per second`).
- `+ - * /` and `+= -= *= /=` keep the engine's fixed-point scale automatically (`2 * 3` is `6`, `6 / 2` is `3`).

### 3.2 Time and continuous blocks

```uw
// Runs every frame while the condition holds; `otherwise` runs when it does not
while key.held(alt) and player.agent.speed > 0.5 and player.agent.stamina > 0 {
    player.agent.speed_modifier = 150%
    player.agent.stamina -= 35 per second
}
otherwise {
    player.agent.speed_modifier = 100%
}

// On the campaign map, every 6 game hours
every 6 hours {
    troop.player.add_gold(50)
}

// A named delayed block that can be cancelled
after 3 days as truce_ends {
    // runs once, 3 game days from now
}
```

Cooldowns need no keyword: compare against `mission.time` (scene clock, seconds) or `game.current_hours` (campaign clock), for example `player.agent.rested_at = mission.time + 2 seconds`.

### 3.3 Events and functions

```uw
fn give_reward(amount) {
    troop.player.add_gold(amount)
    return amount * 2
}

on game.start {
    let total = give_reward(250)
}

on menu.start_game_0.enter {
    scene.town_6_center.jump(0)
    screen.mission()
}

on key.pressed(q) {
    display.message("Q pressed")
}
```

- A function in another feature file is called with the file name: `stamina.refill(20)`.
- A function takes at most 15 parameters (a limit of the Warband virtual machine).

### 3.4 HUD elements

```uw
ui.bar stamina_bar {
    value    = player.agent.stamina
    max      = 100
    position = bottom_left
    size     = (220, 14)
    color    = #3FA34D
    label    = "Stamina"
}
```

All `ui.*` parts are collected into a single shared presentation (`prsnt_uw_hud`); if a full-screen window closes it, the frame dispatcher reopens it.

### 3.5 The engine's own words

UW does not invent engine words. Every record and operation of the Module System kit is available through dotted names:

- Things: `troop.player`, `party.main_party`, `party.town_6`, `scene.town_6_center`, `faction.kingdom_1`, `item.practice_sword`, `menu.town`.
- Constants: `ca.strength`, `skl.ironflesh`, `wpt.one_handed_weapon`.
- Properties, queries and actions:
  - read and write: `player.agent.hit_points`, `player.agent.speed_modifier = 120%`
  - queries and conditions: `let gold = troop.player.gold`, `if troop.player.has_item_equipped(item.practice_sword) { ... }`
  - actions: `troop.player.add_item(item.practice_sword, 0)`, `party.main_party.relocate_near_party(party.town_6, 1)`

`uw words` lists every name a `.uw` file may use.

## 4. Command-line tool

| Command | Purpose |
|---|---|
| `uw build [dir]` | Check the project, compile it with the kit in an isolated staging copy, print the cost report and install into Warband `Modules/` (`--no-install` skips installing). |
| `uw check [dir]` | Fast error check without building (`--json` is used by the editor extension). |
| `uw view` | Measure how much of Native's operation blocks the core language can write. |
| `uw coverage` | Show test coverage of tables, hooks and operations (`--all` for details). |
| `uw explain <name>` | Explain a Native record, who refers to it, and its translations. |
| `uw browse [search]` | Interactive terminal browser of Native data. |
| `uw find "<text>"` | Find every record and translation that contains a name or sentence. |
| `uw words` | List and search the vocabulary generated from the kit. |
| `uw rename <record> "<new name>"` | Rename a record and every text that mentions it, with backups. |

Run `uw --help` for the complete list and options.
