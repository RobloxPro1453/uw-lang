//! Shared by the core tests that go through the real parser: a small inline atlas (the tests must not
//! depend on a kit dump) and lowering helpers.
#![allow(dead_code)]

use uw_atlas::Atlas;
use uw_core::lower::{lower, CoreData};
use uw_core::{parse, Error, Lowered};

fn op_entry(name: &str, code: u32, lhs: bool, can_fail: bool) -> String {
    format!(
        r#"{{"name": "{name}", "code": {code}, "line": null, "signature": "", "lhs": {lhs}, "can_fail": {can_fail}, "global_lhs": {lhs}}}"#
    )
}

pub fn atlas() -> Atlas {
    let lhs = [
        "assign", "val_add", "val_sub", "val_mul", "val_div", "val_clamp", "val_min", "val_max", "store_add",
        "store_sub", "store_mul", "store_div", "store_script_param", "get_player_agent_no", "agent_get_slot",
        "store_current_hours", "store_mission_timer_c_msec", "position_get_x", "position_get_y", "position_get_z", "val_abs", "get_distance_between_positions", "store_agent_hit_points", "create_mesh_overlay", "create_text_overlay",
    ];
    let conds = [
        "eq", "neq", "lt", "le", "gt", "ge", "key_is_down", "key_clicked", "game_key_is_down", "game_key_clicked",
        "is_presentation_active", "agent_is_alive",
    ];
    let plain = ["try_begin", "else_try", "try_end", "try_for_range", "agent_set_slot", "agent_set_speed_modifier", "call_script", "troop_set_name",
        "set_fixed_point_multiplier", "position_set_x", "position_set_y", "overlay_set_position", "overlay_set_size",
        "overlay_set_color", "overlay_set_alpha", "overlay_set_display", "overlay_set_text", "start_presentation",
        "presentation_set_duration", "agent_get_speed", "init_position", "agent_set_hit_points",
        // the start flow (D4b)
        "str_store_string", "troop_set_face_keys", "player_set_face_keys", "party_relocate_near_party",
        "set_jump_mission", "jump_to_scene", "change_screen_mission", "change_screen_map",
        "troop_raise_attribute", "jump_to_menu",
        // text values (D5)
        "str_store_troop_name", "str_store_party_name", "str_store_date", "party_set_name",
    ];
    let mut ops = Vec::new();
    let mut code = 1;
    for name in lhs {
        ops.push(op_entry(name, code, true, false));
        code += 1;
    }
    for name in conds {
        ops.push(op_entry(name, code, false, true));
        code += 1;
    }
    for name in plain {
        ops.push(op_entry(name, code, false, false));
        code += 1;
    }
    let json = format!(
        r#"{{
  "schema": 1,
  "ops": [{}],
  "hooks": ["game_start"],
  "ranges": [],
  "constants": [
    {{"name": "slot_agent_health", "value": 3, "module": "module_constants.py", "line": null}},
    {{"name": "trp_player", "value": 0, "module": "module_troops.py", "line": null}},
    {{"name": "key_left_alt", "value": 56, "module": "header_triggers.py", "line": null}},
    {{"name": "key_q", "value": 16, "module": "header_triggers.py", "line": null}},
    {{"name": "gk_kick", "value": 38, "module": "header_triggers.py", "line": null}},
    {{"name": "pos1", "value": 1, "module": "header_common.py", "line": null}},
    {{"name": "pos61", "value": 61, "module": "header_common.py", "line": null}},
    {{"name": "ca_strength", "value": 0, "module": "header_troops.py", "line": null}}
  ],
  "tables": {{
    "troops": {{
      "module": "module_troops.py", "var": "troops", "id_file": null, "id_prefix": "trp", "ref_tag": "trp",
      "entries": [{{"index": 0, "name": "player", "id": "trp_player", "dup": null, "line": null, "value": "trp_player"}}]
    }},
    "parties": {{
      "module": "module_parties.py", "var": "parties", "id_file": null, "id_prefix": "p", "ref_tag": "p",
      "entries": [{{"index": 0, "name": "main_party", "id": "p_main_party", "dup": null, "line": null, "value": "p_main_party"}}, {{"index": 1, "name": "town_6", "id": "p_town_6", "dup": null, "line": null, "value": "p_town_6"}}]
    }},
    "scenes": {{
      "module": "module_scenes.py", "var": "scenes", "id_file": null, "id_prefix": "scn", "ref_tag": "scn",
      "entries": [{{"index": 0, "name": "town_6_center", "id": "scn_town_6_center", "dup": null, "line": null, "value": "scn_town_6_center"}}]
    }},
    "mission_templates": {{
      "module": "module_mission_templates.py", "var": "mission_templates", "id_file": null, "id_prefix": "mt", "ref_tag": "mt",
      "entries": [{{"index": 0, "name": "town_center", "id": "mt_town_center", "dup": null, "line": null, "value": "mt_town_center"}}]
    }},
    "menus": {{
      "module": "module_game_menus.py", "var": "menus", "id_file": null, "id_prefix": "mnu", "ref_tag": "mnu",
      "entries": [{{"index": 0, "name": "start_game_0", "id": "mnu_start_game_0", "dup": null, "line": null, "value": "mnu_start_game_0"}}, {{"index": 1, "name": "start_phase_2", "id": "mnu_start_phase_2", "dup": null, "line": null, "value": "mnu_start_phase_2"}}]
    }},
    "meshes": {{
      "module": "module_meshes.py", "var": "meshes", "id_file": null, "id_prefix": "mesh", "ref_tag": "mesh",
      "entries": [{{"index": 0, "name": "white_plane", "id": "mesh_white_plane", "dup": null, "line": null, "value": "mesh_white_plane"}}]
    }}
  }}
}}"#,
        ops.join(",\n    ")
    );
    Atlas::from_str(&json).expect("the inline atlas is valid")
}

pub fn parse_files(sources: &[(&str, &str)]) -> Vec<uw_core::ast::File> {
    sources
        .iter()
        .map(|(feature, text)| {
            parse(feature, &format!("features/{feature}.uw"), text)
                .unwrap_or_else(|errors| panic!("{feature} does not parse: {errors:?}"))
        })
        .collect()
}

pub fn try_lower(sources: &[(&str, &str)]) -> Result<Lowered, Vec<Error>> {
    let data = CoreData::builtin().expect("the committed properties.toml is valid");
    lower(&parse_files(sources), &atlas(), &data)
}

pub fn lower_ok(sources: &[(&str, &str)]) -> Lowered {
    try_lower(sources).unwrap_or_else(|errors| panic!("lowering failed: {errors:?}"))
}

/// The one error a source gives.
pub fn lower_err(sources: &[(&str, &str)]) -> String {
    match try_lower(sources) {
        Ok(lowered) => panic!("expected an error, got {lowered:?}"),
        Err(errors) => {
            assert_eq!(errors.len(), 1, "{errors:?}");
            errors[0].message.clone()
        }
    }
}

