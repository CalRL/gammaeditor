pub mod party;

#[macro_export]
macro_rules! tauri_commands {
    () => {
        tauri::generate_handler![

            //player commands
            gammaeditor_lib::save::player::get_player_transform,
            gammaeditor_lib::save::player::get_player_money,
            gammaeditor_lib::save::player::get_player_direction,
            gammaeditor_lib::save::player::get_player_position,
            gammaeditor_lib::save::player::get_trainer_name,
            gammaeditor_lib::save::player::get_trainer_gender,

            gammaeditor_lib::file::save::is_save_loaded,

            //backup commands
            gammaeditor_lib::save::backup::create_backup,

            // boxes commands
            gammaeditor_lib::save::boxes::get_box,
            gammaeditor_lib::save::boxes::get_enriched_box,
            gammaeditor_lib::save::boxes::get_enriched_box_with_info,
            gammaeditor_lib::save::boxes::get_enriched_box_grid,
            gammaeditor_lib::save::boxes::get_enriched_mon_list,
            gammaeditor_lib::save::boxes::get_pos_array,
            gammaeditor_lib::save::boxes::check_pos_index,
            gammaeditor_lib::save::boxes::get_box_size,
            gammaeditor_lib::save::boxes::get_pp_by_index,
            gammaeditor_lib::save::boxes::get_pp,
            gammaeditor_lib::save::boxes::get_moves_by_index,
            gammaeditor_lib::save::boxes::get_moves_by_box,
            gammaeditor_lib::save::boxes::get_ivs_by_index,
            gammaeditor_lib::save::boxes::get_ivs,
            gammaeditor_lib::save::boxes::get_genders_by_box,
            gammaeditor_lib::save::boxes::get_enriched_moves,
            gammaeditor_lib::save::boxes::get_xp_by_index,

            gammaeditor_lib::save::boxes::get_class_by_index,
            gammaeditor_lib::save::boxes::get_classes_by_box,

            gammaeditor_lib::save::boxes::grid::get_simple_mon_grid,

            // party
            gammaeditor_lib::save::party_legacy::get_enriched_party,
            gammaeditor_lib::save::party_legacy::get_pokemon_info,
            gammaeditor_lib::save::party_legacy::get_party,

            gammaeditor_lib::save::box_edit::set_shiny_by_index,

        ]
    };
}