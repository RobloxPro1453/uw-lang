# graph_viewer data format: `uw-flow/1`

One UTF-8 JSON object. Produced today by `py/flow_export.py` (menus only); later by the compiler
(`uw graph --json`) with more node kinds. A viewer must ignore fields it does not know.

```json
{
  "format": "uw-flow/1",
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
      "on_enter_goes_to": [],
      "options": [
        {"id": "mno_start_male", "text": "Male", "conditional": false,
         "goes_to": [{"op": "jump_to_menu", "to": "menu_start_character_1"}]},
        {"id": "mno_go_back", "text": "Go back", "conditional": false,
         "goes_to": [{"op": "jump_to_menu", "to": "menu_start_game_0"}]}
      ]
    }
  ],
  "edges": [
    {"from": "menu_start_game_1", "to": "menu_start_character_1", "op": "jump_to_menu",
     "via_option": "mno_start_male"}
  ]
}
```

## Fields

| Field | Meaning |
|---|---|
| `roots[].id` | Where the game itself starts a flow; default starting points of the view. |
| `nodes[].id` | Unique internal name; what the modder writes in a `.uw` file. |
| `nodes[].kind` | `menu` today; later also `script`, `dialog`, `mission_template`, `presentation`, `screen`. |
| `nodes[].text` | In-game text of the screen; may contain game markup like `{s10}`, `^` (= new line), `{!}` (= untranslated). Show `^` as a line break, keep `{...}` as-is in grey. |
| `nodes[].source` | File and line in the Module System where it is defined. |
| `nodes[].on_enter_goes_to` | Where the screen's opening code sends the player before any button (e.g. straight to the map). |
| `nodes[].options[]` | The screen's buttons in display order: `id`, `text`, `conditional` (button only shown when a condition holds), `goes_to` (what pressing it does). |
| `edges[]` | Every arrow once more as a flat list: `from`, `to`, `op`, `via_option` (button id or null = opening code). |
| `to` values | A node id (`menu_...`, `script_...`, `prsnt_...`, `scn_...`, `mst_...`) or a screen: `screen:map`, `screen:return`, `screen:quit`, `screen:inventory`, ... (from `change_screen_*`). Targets that are not menus may have no node yet; draw them as small grey pills. |
| `op` | The game operation behind the arrow. Plain words: `jump_to_menu` opens, `change_screen_map` goes to the world map, `change_screen_return` closes the menu, `call_script` runs code, `start_presentation` opens a custom screen, `jump_to_scene` / `set_jump_mission` start a scene. |

## Planned additions (same format version, optional fields)

- `nodes[].text_tr`: official Turkish text (from `Modules/Native/languages/tr`).
- `impact`: `[{"id": "...", "why": "no longer reached from anywhere"}]` for a modded flow.
- `changed`: `"added" | "removed" | "changed"` on nodes/edges in a mod flow.
- `nodes[].kind = "dialog"` with `speaker`, `state_in`, `state_out`.
