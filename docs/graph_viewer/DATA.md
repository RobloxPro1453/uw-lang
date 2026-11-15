# graph_viewer data format: `uw-flow/1`

One UTF-8 JSON object, exact rules in `schema.json`. Produced today by `py/flow_export.py` (menus with
their options; every arrow target that is not a menu is a small `stub` node); later by the compiler
(`uw graph --json`) with more node kinds.

**Compatibility promise:** inside `uw-flow/1` fields are only added, never renamed, removed or
retyped. Every listed field is always present (empty string / empty list / null when there is nothing).
A viewer ignores fields it does not know and draws unknown `kind` / `op` values generically.

```json
{
  "format": "uw-flow/1",
  "generator": "py/flow_export.py",
  "kit_hash": "9f56b9fd...",
  "roots": [
    {"id": "menu_start_game_0", "why": "menu #0: the engine opens it on New Game"},
    {"id": "menu_start_phase_2", "why": "the engine opens it after character creation (D4)"}
  ],
  "nodes": [
    {
      "id": "menu_start_game_1",
      "kind": "menu",
      "index": 7,
      "text": "Select your character's gender.",
      "source": "module_game_menus.py:928",
      "stub": false,
      "on_enter_goes_to": [],
      "options": [
        {"index": 0, "id": "mno_start_male", "text": "Male", "conditional": false,
         "goes_to": [{"op": "jump_to_menu", "to": "menu_start_character_1"}]},
        {"index": 2, "id": "mno_go_back", "text": "Go back", "conditional": false,
         "goes_to": [{"op": "jump_to_menu", "to": "menu_start_game_0"}]}
      ]
    },
    {"id": "screen:map", "kind": "screen", "index": -1, "text": "", "source": "", "stub": true,
     "on_enter_goes_to": [], "options": []}
  ],
  "edges": [
    {"from": "menu_start_game_1", "to": "menu_start_character_1", "op": "jump_to_menu",
     "via_option": "mno_start_male", "option_index": 0}
  ]
}
```

## Fields

| Field | Meaning |
|---|---|
| `roots[].id` | Where the game itself starts a flow; default starting points of the view. |
| `nodes[].id` | Unique internal name; what the modder writes in a `.uw` file. |
| `nodes[].kind` | Today: `menu` (full card), and as stubs `script`, `mission_template`, `scene`, `presentation`, `screen`. Later also `dialog` and others; unknown kinds are drawn as generic pills. |
| `nodes[].text` | In-game text of the screen; may contain game markup like `{s10}`, `^` (= new line), `{!}` (= untranslated). Show `^` as a line break, keep `{...}` as-is in grey. |
| `nodes[].source` | File and line in the Module System where it is defined. |
| `nodes[].on_enter_goes_to` | Where the screen's opening code sends the player before any button (e.g. straight to the map). |
| `nodes[].stub` | `true`: the node exists only as an arrow target (script, scene, `screen:map`...), no text/options yet; draw small. |
| `nodes[].options[]` | The screen's buttons in display order: `index` (0-based, unique in the node), `id` (may repeat inside a node!), `text`, `conditional` (button only shown when a condition holds), `goes_to` (what pressing it does). |
| `edges[]` | Every arrow once more as a flat list: `from`, `to` (both always node ids), `op`, `via_option` (button id or null = opening code), `option_index` (which button, or null). |
| `to` values | A node id: `menu_...`, `script_...`, `prsnt_...`, `scn_...`, `mst_...` or a screen `screen:map`, `screen:return`, `screen:trade`, ... (from `change_screen_*`). Non-menu targets are `stub` nodes. |
| `op` | The game operation behind the arrow. Plain words: `jump_to_menu` opens, `change_screen_map` goes to the world map, `change_screen_return` closes the menu, `call_script` runs code, `start_presentation` opens a custom screen, `jump_to_scene` / `set_jump_mission` start a scene. |

## Planned additions (same format version, optional fields)

- `nodes[].text_tr`: official Turkish text (from `Modules/Native/languages/tr`).
- `impact`: `[{"id": "...", "why": "no longer reached from anywhere"}]` for a modded flow.
- `changed`: `"added" | "removed" | "changed"` on nodes/edges in a mod flow.
- `nodes[].kind = "dialog"` with `speaker`, `state_in`, `state_out`.
