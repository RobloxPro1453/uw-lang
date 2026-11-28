# graph_viewer — brief for the app builder

A desktop app that shows how a Mount & Blade: Warband mod "flows": which screen (menu) leads to which,
what every button (option) on a screen does, and later what a mod changed. It is a separate project;
this folder is its whole brief:

- `README.md` (this file): what to build and how it must feel.
- `DATA.md`: the input format `uw-flow/1`, field by field.
- `schema.json`: the same format as JSON Schema (draft 2020-12). The app validates input against it.
- `sample_flow.json`: real data (409 nodes, 949 edges), on the owner's machine only (it holds the
  game's texts). Develop and test with it.

## Who uses it and why

A modder who does NOT know the game's internal names. He must see the whole game flow, click a screen
and at once know its internal name (to write in his mod) and what each of its buttons does — without
typing any parameter. A first attempt (one flat Mermaid picture of everything) was unreadable.

## How it must feel: an infinite canvas (visual reference: Obsidian, clean room)

Look and feel like Obsidian's canvas / graph views: a large dark (or light) workspace, smooth and calm.

- **Infinite canvas.** Mouse wheel = zoom around the cursor (smooth, ~0.1x to 3x); hold the middle
  mouse button (or Space + left drag, or right drag) = pan in any direction; trackpad pinch/two-finger
  scroll also work. "Fit all" and "Fit selection" buttons and keys (`F`, `Shift+F`), zoom % shown.
  Like a PDF viewer: you can always drag further to keep reading.
- **Crisp at every zoom:** render as SVG (or canvas with vector text); text never pixelates. At low
  zoom, cards collapse to their title only (level of detail), so a zoomed-out view stays readable.
- **Minimap** in a corner showing where the viewport is; click/drag it to move.

## What is drawn

1. **Opens with no input:** load the file (drag & drop, or a "Open…" button, remember the last file)
   and show the flow from `roots` (first root selected). A searchable list of all nodes lets the user
   start from anywhere.
2. **Layered layout, top to bottom** (Sugiyama style, like dagre/ELK): roots on top, each step a
   level lower. Back-edges (a "Go back" button pointing up) are drawn as curved arrows on the side, the
   target card is not duplicated. Layout is computed once per expansion and animated, not jumping.
3. **Progressive expansion:** at start show 3 levels from the root. Each card has a `+`/`−` to expand
   or collapse its children. "Expand all" is allowed but warns above ~150 cards.
4. **A menu is a card** (rounded, soft shadow), ~280 px wide:
   - title: in-game `text`, first ~90 chars, `^` as line break, `{s10}`-style markup in grey;
   - small mono line: internal `id` with a copy button (this is what the modder writes);
   - its **buttons** (`options`) as rows inside the card, in order: button text, a small "only
     sometimes" badge when `conditional`, and a connector dot on the right edge from which that
     button's arrows leave;
   - arrows of the screen's opening code (`on_enter_goes_to`) leave from the card header.
5. **Stub nodes** (`stub: true`: scripts, scenes, screens like `screen:map`) are small pills with an
   icon per `kind`; they are not expanded further.
6. **Arrows** are smooth curves with an arrowhead and a short plain-words label from `op`:
   jump_to_menu "opens", change_screen_map "world map", change_screen_return "closes",
   call_script "runs", start_presentation "shows", jump_to_scene / set_jump_mission "starts scene";
   unknown op: show it as is. Hovering an arrow highlights it and both ends.
7. **Click a card:** selects it and opens a right side panel: full text, id (copy), source
   (`module_game_menus.py:928`), its buttons with targets, "reached from" (incoming arrows) and
   "leads to" lists; clicking an entry in the panel pans the canvas to that card.
8. **Search** (`Ctrl+K`): by id or words of the text; Enter pans to the card and highlights it.
9. **Later (keep the code ready):** load a second file of the same format (a mod) and color the
   difference: removed / unreachable red, new green, changed orange. Optional `impact` field
   (DATA.md, planned) lists unreachable nodes.

## Must not break when the data grows

- Validate with `schema.json`; show a clear message for an invalid file instead of a blank screen.
- Ignore unknown fields; draw unknown `kind` as a generic pill and unknown `op` with its raw name.
- Never assume a node has options or text (they may be empty); never assume ids are unique for
  options (use `option_index`).
- The `format` string is the only version check: `uw-flow/1` must keep working forever; a future
  `uw-flow/2` may be shown as "newer format, please update".
- Must stay smooth with ~400 nodes / ~1000 edges loaded (only draw what is expanded).

## Where the data comes from

universal_warband repo: `uw graph --json flow.json` (reads the local atlas
`target/uw-staging/atlas.json`; `uw graph --json flow.json start_phase_2` makes that menu the start).
Later the same command also writes a mod's flow with its differences.
