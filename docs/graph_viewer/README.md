# graph_viewer — brief for the app builder

A desktop/web viewer that shows how a Mount & Blade: Warband mod "flows": which screen (menu) leads to
which, what every button (option) on a screen does, and what a mod changed. It is a separate project;
this folder is its whole brief. The data comes from the universal_warband (UW) compiler as one JSON
file (format in [DATA.md](DATA.md)); the viewer only reads it.

## Who uses it and why

The user is a modder who does NOT know the game's internal names. Today the flow is only visible in
code, so he has to guess which internal menu to change (e.g. that the screen after character creation
is `menu_start_phase_2`). The viewer must let him **see the whole game flow at a glance, click a screen
and immediately know its name and what each of its buttons does**, without typing any parameter.

What was wrong with the first attempt (a raw Mermaid chart of everything): one huge flat picture, tiny
unreadable boxes, no way to start "from the beginning", option texts missing.

## Must have

1. **Opens with no input.** Default view: the new-game flow starting from the roots in the JSON
   (`roots`: the first menu, and the screen the engine opens after character creation). The user can
   also pick any screen from a searchable list to start from.
2. **Readable at any size.** Top-to-bottom layered layout; show a few levels at first and expand on
   click (collapse/expand per node); pan and zoom; fit-to-screen button. Never draw all 217 menus at
   once by default.
3. **A screen is a card**, not a tiny box: title = the in-game text (first ~80 chars), small grey line
   = internal id (`menu_start_game_1`) with a copy button, then its **buttons (options) listed inside
   the card**, each with its text ("Male", "Go back") and an arrow to where it leads. A conditional
   button (`conditional: true`) is marked ("only sometimes visible").
4. **Edges explain themselves:** an arrow leaves from the button that causes it; arrows from the
   screen's own opening code (`on_enter_goes_to`) leave from the card header. Edge label in plain
   words: jump_to_menu = "opens", change_screen_map = "goes to the world map", call_script = "runs".
5. **Click a card → side panel:** full text, id, source location (`source`, e.g.
   `module_game_menus.py:928`), all incoming arrows ("reached from"), all outgoing arrows.
6. **Search** by id or by any words of the in-game text (fuzzy is fine).
7. **Mod diff (second step):** load a second JSON (the modded flow) and color what changed: red =
   edge removed / screen no longer reachable, green = new, orange = changed. Optional `impact` list in
   the JSON (see DATA.md) gives the unreachable screens directly.
8. Cycles are normal (a "Go back" button points up); draw them as back-edges, do not duplicate cards.
9. Light and dark theme; fast with ~1000 edges.

## Nice to have

- Export the current view as PNG/SVG.
- Turkish UI later (labels in English are fine at first; the game texts are English, Turkish texts
  may be added to the JSON later as `text_tr`).

## Where the data comes from

In the universal_warband repo: `python py/flow_export.py --out flow.json` (needs the local atlas
`target/uw-staging/atlas.json`, built from the Module System kit). A ready example is
`docs/graph_viewer/sample_flow.json` on the owner's machine (not in git: it contains the game's own
texts). Later the compiler will write the same format directly (`uw graph --json`) and also for a mod.
