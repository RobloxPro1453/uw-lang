# Mining: multiplayer and persistent world

Mined from the 17 mods under `warband-opensource-mods/mods/`. The topic lives in two large custom
protocols (`05-persistent-world`, `10-persistent-kingdoms`), one co-op protocol shipped twice as the
same files (`03-tainted-paths` == `04-medieval-conquests`), one second co-op protocol that is a merged
copy of that one (`07-brytenwalda`), and the rest are Native mission templates with local tweaks.
Evidence is `mods/<NN-name>/<path>:<line>`. `target/ref/kit/` is empty in this worktree, so
"Native vs mod-added" is judged only from in-file comments, `ID_*` constants and the shape of the code.

## Patterns

| # | pattern (generic, one line) | how it is done in MS (2-3 lines, operations named) | mods using it (count of 17 + names) | evidence (file:line, 1-3) |
|---|---|---|---|---|
| 1 | Server-only block: every trigger that writes shared state runs on the server alone | `(multiplayer_is_server)` is placed as the first condition of the trigger's condition block; the engine ANDs conditions left to right, so the agent loops, score writes and mission starts below it never run on a client. Native-derived templates carry the same idea with the `#for only server itself` comment. | 12 — 01, 02, 03, 04, 05, 07, 08, 09, 10, 11, 12, 13 | `mods/05-persistent-world/module_mission_templates.py:168`<br>`mods/01-anno-domini-1257/module_mission_templates.py:3721`<br>`mods/04-medieval-conquests/src/module_coop_mission_templates.py:42` |
| 2 | Server push to everyone: loop over player ids and send per player | `(get_max_players, ":num_players")` then `(try_for_range, ":player_no", 1, ":num_players")` with `(player_is_active, ...)`, then one `multiplayer_send_N_int_to_player` call inside. The comment `#0 is server so starting from 1` is copy-pasted at every such loop. | 5 — 01, 02, 05, 07, 10 | `mods/02-twilight-of-the-sun-king/module_mission_templates.py:9704`<br>`mods/01-anno-domini-1257/module_mission_templates.py:12869`<br>`mods/07-brytenwalda/source/multiplayer/scripts.py:2703` |
| 3 | Guard a trigger's side effects for "server or single player" | `(this_or_next\|multiplayer_is_server), (neg\|game_in_multiplayer_mode)` as the first two conditions, so the same file behaves correctly when loaded as a single-player mod and as a multiplayer server. The same three lines are repeated in every destructible-scene-prop trigger. | 6 — 01, 02, 11, 12, 13, 14 | `mods/11-diplado/source/module_mission_templates.py:264`<br>`mods/12-beggars-diplomacy/source/module_scene_props.py:374`<br>`mods/13-catplomacy/module_mission_templates.py:1028` |
| 4 | Change the mission from script, server side only | Resolve the template with `script_game_multiplayer_get_game_type_mission_template`, then `(start_multiplayer_mission, reg0, <scene>, 1)`; the old mission is closed with `script_game_set_multiplayer_mission_end`. Never called directly from a key press, always from a poll end or a network handler. | 5 — 01, 03, 04, 05, 09 | `mods/04-medieval-conquests/src/module_coop_scripts.py:1692`<br>`mods/05-persistent-world/module_mission_templates.py:519`<br>`mods/09-between-empires/SourceCode/module_mission_templates.py:7402` |
| 5 | One receive script per side with a growing if/else chain on the event type | The engine calls one script with sender, event type and up to four values; every message is one `(try_begin)` / `(else_try)` branch keyed on `:event_type`, with `(store_script_param, ..., 3..6)` used to unpack the payload registers. | 5 — 03, 04, 05, 07, 10 | `mods/05-persistent-world/module_scripts.py:442`<br>`mods/10-persistent-kingdoms/module_scripts.py:1031`<br>`mods/07-brytenwalda/source/multiplayer/coop_scripts.py:812` |
| 6 | Client sends the intent, server validates and applies it | The client only calls `multiplayer_send_*_to_server` with the request; the server branch re-checks the sender against the world (`prop_instance_is_valid`, distance, cooldown, `player_is_admin`) and only then writes state and fans out the result. | 5 — 03, 04, 05, 07, 10 | `mods/07-brytenwalda/source/multiplayer/shield_bash.py:17`<br>`mods/07-brytenwalda/source/multiplayer/scripts.py:2685`<br>`mods/05-persistent-world/module_scripts.py:904` |
| 7 | Vote / poll resolution in a periodic server trigger | `$g_multiplayer_poll_running` plus vote counters in global registers; a repeating `(1, 5, 0, ...)` trigger compares the mission timer and the sent-vote count, calls `script_cf_multiplayer_evaluate_poll`, then changes map or faction. Servers with their own poll system keep the same shape but call `script_check_polls_ended`. | 5 — 01, 02, 08, 09, 10 | `mods/01-anno-domini-1257/module_mission_templates.py:3739`<br>`mods/08-money-and-honour/src/module_mission_templates.py:1348`<br>`mods/10-persistent-kingdoms/module_mission_templates.py:664` |
| 8 | Partition the bounded event-id space by direction | `header_common` declares `server_event_*` from 0 upwards, then a "add new events here" line, then `client_event_*` starting at 95 / 100 / 113, all below the engine ceiling of 127. The remaining ids are what the mod actually has to spend, so the split is documented in a comment. | 4 — 03, 04, 05, 10 | `mods/05-persistent-world/header_common.py:39`<br>`mods/05-persistent-world/header_common.py:64`<br>`mods/04-medieval-conquests/src/header_common.py:125` |
| 9 | Replicate generic "slot" writes instead of one message per field | One event per entity kind — `server_event_player_set_slot`, `_scene_prop_set_slot`, `_faction_set_slot`, `_troop_set_slot`, `_agent_set_slot` — carrying entity id, slot index and value; the receiver just calls the matching `*_set_slot`, so new replicated fields need no new event. | 4 — 03, 04, 05, 10 | `mods/05-persistent-world/header_common.py:12`<br>`mods/05-persistent-world/module_scripts.py:3459`<br>`mods/10-persistent-kingdoms/module_scripts.py:4443` |
| 10 | One generic command channel: one event type + an integer command id | `server_event_return_game_rules` and `client_event_set_game_rules` carry `<command_id>, <value>`; a shared `script_cf_execute_command` switches on the `command_*` ids on both sides, so a "get" and its "set" reply share one table instead of two event ids per setting. | 4 — 03, 04, 05, 10 | `mods/05-persistent-world/module_scripts.py:316`<br>`mods/05-persistent-world/module_scripts.py:743`<br>`mods/05-persistent-world/header_common.py:202` |
| 11 | Admin action = a numeric action id routed to a server-side checker | The client panel sends `admin_action_*` id + target + values; the server script `cf_admin_action` re-checks `player_is_admin`, the sender's per-action permission slot and the target's existence before acting, and logs the result. | 4 — 03, 04, 05, 10 | `mods/10-persistent-kingdoms/module_scripts.py:13617`<br>`mods/05-persistent-world/module_scripts.py:1163`<br>`mods/04-medieval-conquests/src/module_coop_presentations.py:938` |
| 12 | Join snapshot plus a "client ready" callback | `ti_server_player_joined` on the server pushes the whole world state to that one player only (rules, player/faction slots, scores, attached scene props); the client then runs `script_after_client_is_setup`, which the engine fires when the initial data updates are done. | 3 — 01, 05, 10 | `mods/05-persistent-world/module_mission_templates.py:183`<br>`mods/05-persistent-world/module_scripts.py:3562`<br>`mods/10-persistent-kingdoms/module_mission_templates.py:212` |
| 13 | Throttle / cooldown with a stored timestamp | Client stores `store_mission_timer_a` in a scene-prop or troop slot and skips the request until `now > last + interval`; the server keeps its own counter or cooldown slot and rejects anything faster, decrementing it from a periodic trigger. | 3 — 05, 07, 10 | `mods/05-persistent-world/module_scripts.py:43`<br>`mods/10-persistent-kingdoms/module_scripts.py:612`<br>`mods/07-brytenwalda/source/multiplayer/shield_bash.py:20` |
| 14 | Add a game mode without touching the engine | A new numeric id (`multiplayer_game_type_coop_battle = 8`, `multiplayer_num_game_types = 10`) plus a rewritten `script_game_multiplayer_get_game_type_mission_template` that returns the mod's own mission-template id. | 3 — 03, 04, 05 | `mods/04-medieval-conquests/src/header_common.py:157`<br>`mods/05-persistent-world/module_scripts.py:1556`<br>`mods/04-medieval-conquests/src/module_coop_scripts.py:1692` |
| 15 | One trigger, two halves: client work first, server work after a mid-block guard | The trigger runs on both sides and the client-only calls happen first; a bare `(multiplayer_is_server),` then turns the rest of the block into server-only code. It is the same idea as `try_begin` / `else_try` but written with one condition line. | 2 — 05, 10 | `mods/05-persistent-world/module_mission_templates.py:199`<br>`mods/10-persistent-kingdoms/module_mission_templates.py:285` |
| 16 | Persist per-player state in a hidden troop used as an array | `trp_inactive_players_array` holds fixed-size records (`player_data_size` stride slots), keyed by `player_get_unique_id` (server-side only); written by `script_cf_save_player_exit` on `ti_on_player_exit` and read back when the same account joins again. | 2 — 05, 10 | `mods/10-persistent-kingdoms/module_troops.py:142`<br>`mods/10-persistent-kingdoms/module_scripts.py:4557`<br>`mods/05-persistent-world/module_scripts.py:3580` |
| 17 | Sequenced and acknowledged channel built on the event id itself | Chat uses a dedicated event-id range as a sequence number: the server tracks the next expected number per player, the client keeps the unacked message in a troop slot and a 0.3 s trigger resends it until the ack arrives. Out-of-order arrivals are detected by comparing the number. | 2 — 05, 10 | `mods/05-persistent-world/module_scripts.py:2345`<br>`mods/05-persistent-world/module_mission_templates.py:772`<br>`mods/05-persistent-world/module_scripts.py:706` |
| 18 | Two correlated messages when an int must tag a string | The sender first sends the integer context (faction id, event type) with an int message, then the text with `multiplayer_send_string_*`; the receiver remembers the id in a troop/player slot and matches it when the string message arrives. | 2 — 05, 10 | `mods/05-persistent-world/module_scripts.py:931`<br>`mods/05-persistent-world/module_scripts.py:1005`<br>`mods/05-persistent-world/module_scripts.py:726` |
| 19 | Bit-pack several small values into one 32-bit payload | Shift/mask constants live in `header_common` (`net_sound_shift`, `net_pack_3_*`) and the send site does `val_lshift` / `val_or` by hand; positions and packed pos+angle are compressed into one int with the comment that it is "for a smaller network message". | 2 — 05, 10 | `mods/05-persistent-world/module_scripts.py:474`<br>`mods/05-persistent-world/header_common.py:219`<br>`mods/10-persistent-kingdoms/header_common.py:244` |
| 20 | Filter recipients by distance and line of sight before sending | The server computes `get_distance_between_positions` (and line-of-sight / behind checks) from the recipient's agent and sends only to players whose agent is near; the rest never see the sound or the state change. | 2 — 05, 10 | `mods/05-persistent-world/module_scripts.py:3894`<br>`mods/10-persistent-kingdoms/module_scripts.py:1659` |
| 21 | Log every privileged or economy action server side | `(server_add_message_to_log, "str_log_...")` at the action site, with `str_store_player_username` / `str_store_faction_name` filling the placeholders; the same helper writes server console lines and the log file. | 2 — 05, 10 | `mods/05-persistent-world/module_scripts.py:1018`<br>`mods/10-persistent-kingdoms/module_scripts.py:247`<br>`mods/10-persistent-kingdoms/module_scripts.py:1686` |
| 22 | Accounts and name checks outside the engine over HTTP | Build a URL string with `str_encode_url`, send it with `send_message_to_url`; the reply arrives in `script_game_receive_url_response` (integer_count, string_count, then ints and strings) and is matched back to the player by name and `player_get_unique_id`. | 2 — 05, 10 | `mods/05-persistent-world/module_scripts.py:371`<br>`mods/05-persistent-world/module_scripts.py:3810`<br>`mods/10-persistent-kingdoms/module_scripts.py:4804` |
| 23 | Third-party protocol merged in by copy-paste | A whole dispatch block is pasted into the host mod's receive script, with a comment telling the user to paste it "right under the store_script_param's"; while merging, the foreign `(multiplayer_is_server)` gate is left commented out at the top of the block. | 2 — 03/04, 07 | `mods/07-brytenwalda/source/multiplayer/scripts.py:2674`<br>`mods/07-brytenwalda/source/multiplayer/coop_scripts.py:818`<br>`mods/04-medieval-conquests/src/module_coop_mission_templates.py:1207` |

## Engine facts learned

- Network event ids are capped at 0..127 by the engine; every mod's `header_common` repeats this as a
  comment and plans its id layout around it — `mods/05-persistent-world/header_common.py:64`,
  `mods/10-persistent-kingdoms/header_common.py:85`.
- A message carries at most four integers OR one string, nothing structured: the only send operations
  are `multiplayer_send_{,1..4}_int_to_{server,player}` and `multiplayer_send_string_to_*` —
  `mods/05-persistent-world/header_operations.py:123`.
- The receiving script gets `(player/sender, event_type, value, value_2, value_3, value_4)` and is the
  same script on client and server, so it must branch on `multiplayer_is_server` itself —
  `mods/05-persistent-world/module_scripts.py:446`, `mods/07-brytenwalda/source/multiplayer/coop_scripts.py:811`.
- Player id 0 is the server, so fan-out loops start at 1 (`#0 is server so starting from 1`) —
  `mods/02-twilight-of-the-sun-king/module_mission_templates.py:9704`.
- Delivery is not ordered and not acknowledged: a mod has to keep its own "next expected event number"
  and a resend timer, otherwise chat lines are lost or reordered —
  `mods/05-persistent-world/module_scripts.py:935`, `mods/05-persistent-world/module_mission_templates.py:772`.
- Strings on the wire are module string ids, not text; the engine/localisation resolves them, which is
  why a server can send a canned message and every client shows it in its own language —
  `mods/05-persistent-world/module_scripts.py:448`.
- `preset_message_*` flags turn a raw value into a name client side (item, agent, player, faction,
  castle), so "X killed Y" costs three ints instead of a string —
  `mods/10-persistent-kingdoms/header_common.py:88`, `mods/05-persistent-world/header_common.py:67`.
- `player_get_unique_id` can only be used server side and is stable across sessions: it is the only
  usable account key — `mods/05-persistent-world/header_operations.py:177`.
- Engine limits that leak into design: 2..250 players, 2 teams plus hard-coded spectators, respawn
  period 3..31 s because dead agents are removed after roughly 31 s —
  `mods/10-persistent-kingdoms/header_common.py:231`, `mods/10-persistent-kingdoms/header_common.py:234`,
  `mods/10-persistent-kingdoms/header_common.py:236`.
- `after_client_is_setup` is a client-side engine callback fired when the server has finished sending
  the initial data updates — the only reliable "join handshake finished" hook —
  `mods/05-persistent-world/module_scripts.py:3574`.
- Payload values are 32-bit signed integers, so a packed value must stay under `1 << 31` —
  `mods/10-persistent-kingdoms/header_common.py:240`.
- The game-rules "command" ids are one shared integer space with holes: engine/module-system ids 1..100,
  hard-coded engine ids 101..124, then 130..136 used by one mod, each range bracketed by `*_begin` /
  `*_end` marker constants — `mods/05-persistent-world/header_common.py:202`.
- HTTP is fire-and-forget and asynchronous: `send_message_to_url` and a later
  `script_game_receive_url_response` with integer_count/string_count — `mods/05-persistent-world/module_scripts.py:371`.
- A community engine fork adds a composite/binary message API (`multiplayer_message_init`,
  `multiplayer_message_put_int/string/position`, `multiplayer_send_composite_message_to_*`, needs
  `network_compatible = 0`) but no mod in this set uses it, and the same operation has opcode 3405 in
  one fork and 3406 in the other — `mods/03-tainted-paths/src/header_operations.py:3107`,
  `mods/04-medieval-conquests/src/header_operations.py:1926`.

## Pain points

- Extending the protocol means editing three places by hand: the id table in `header_common`, the
  dispatcher branch, and the sender. There is no declaration to keep them in sync, and a wrong id
  fails silently (the branch simply never runs).
- The 128-id space is nearly exhausted by Native: in one mod the co-op events had to start at 113
  because everything below was taken — `mods/04-medieval-conquests/src/header_common.py:125`. Ids are
  also hand-numbered, and one of the two big mods already assigned 120 twice
  (`client_event_chat_message_end` and `client_event_chat_message_type`) —
  `mods/05-persistent-world/header_common.py:55`.
- No structured messages: "an id plus a string", "a position plus an angle", "a type plus three
  numbers" all become either two messages or hand-rolled bit packing with shift/mask constants.
- No reliable channel: chat had to become the transport (sequence numbers, ack, resend, and a "received
  up to N" cursor), and unrelated features are deliberately routed through it "for more reliability" —
  `mods/10-persistent-kingdoms/module_scripts.py:1667`.
- The client/server split is enforced by convention only. A forgotten `multiplayer_is_server` runs
  server writes on clients, so mods accumulate dozens of commented-out guards while debugging —
  `mods/04-medieval-conquests/src/module_coop_mission_templates.py:1207`,
  `mods/07-brytenwalda/source/multiplayer/coop_scripts.py:818`.
- There is no plugin point: the handler is one script, so a third-party feature ships with paste
  instructions ("paste this block right under the store_script_param's") —
  `mods/07-brytenwalda/source/multiplayer/scripts.py:2674`.
- Admin tools are written twice — a client presentation that only sends messages, and a server switch
  that re-checks permission, target validity and cooldown for every action —
  `mods/05-persistent-world/module_presentations.py:852` vs `mods/05-persistent-world/module_scripts.py:1163`.
- Permissions come from outside the engine, so they are unpacked into one player slot per action, with
  0 meaning "allowed", so an unreachable name server silently makes everybody an admin —
  `mods/05-persistent-world/module_constants.py:61`, `mods/05-persistent-world/module_scripts.py:3834`.
- Persistence is hand-rolled: hidden troops used as arrays (`inactive_players_array`, `mission_data`,
  `temp_array`, `chat_overlay_ring_buffer_0/1`), stride arithmetic, save on exit, reload on join —
  `mods/05-persistent-world/module_troops.py:139`.
- Anything that must survive a server restart needs a second system outside the engine (URL requests,
  an own account service, a deferred kick timer so the player can read the rejection message) —
  `mods/05-persistent-world/module_scripts.py:421`, `mods/05-persistent-world/module_constants.py:56`.
- Join and leave bookkeeping (give the new player the world, then react to "ready", save on exit) is
  boilerplate that every persistent-world mod rewrites from scratch —
  `mods/05-persistent-world/module_scripts.py:3562`, `mods/10-persistent-kingdoms/module_scripts.py:4557`.

## For UW

- Make the client/server split part of the type system: a value or function is server-only, client-only
  or replicated, and the compiler emits the guards; touching authoritative state from client code should
  not compile instead of failing silently at runtime.
- Give messages a first-class declaration (name, direction, typed fields, optional string) and generate
  the id assignment, the packing, the sending side and both dispatch branches from it; a duplicate or
  out-of-range id becomes a compile error.
- Provide reliable, ordered messaging as a built-in verb — sequence numbers, acknowledgement, retry and
  de-duplication handled by the runtime — so no feature has to be smuggled through the chat channel.
- Treat a player's persistent state as a declared record; generate the join snapshot, per-field delta
  replication, save on exit, reload on rejoin and the key from the engine's stable player identity.
- Offer recipient selection as general verbs (`broadcast`, `send_to`, `send_near`, `send_in_view`) with
  filtering done by the compiler, not by a hand-written distance loop in every feature.
- Make rate limiting and cooldowns declarative properties of a message or action, checked on the server
  automatically.
- Model admin rights as capabilities on the player type; generate the permission check, the server-side
  re-check and the audit log line from one declaration.
- Expose persistence as a keyed store with generated save/load hooks and an external account hook, so
  the engine's own save system is not abused as a data structure.
- Keep an escape hatch to raw engine event ids and raw slots: the 128-event, four-int payload limits
  still apply, and a high-level protocol layer must degrade gracefully when the target is the stock
  engine.
