#[macro_export]
macro_rules! tauri_commands {
    () => {
        tauri::generate_handler![

            //player commands
            crate::save::player::get_player_transform,
            crate::save::player::get_player_money,
            crate::save::player::get_player_direction,
            crate::save::player::get_player_position,
            crate::save::player::get_trainer_name,
            crate::save::player::get_trainer_gender,

            crate::file::save::is_save_loaded,

            //backup commands
            crate::save::backup::create_backup,

            // boxes commands
            crate::save::boxes::get_box,
            crate::save::boxes::get_enriched_box,
            crate::save::boxes::get_enriched_box_with_info,
            crate::save::boxes::get_enriched_box_grid,
            crate::save::boxes::get_enriched_mon_list,
            crate::save::boxes::get_pos_array,
            crate::save::boxes::check_pos_index,
            crate::save::boxes::get_box_size,
            crate::save::boxes::get_pp_by_index,
            crate::save::boxes::get_pp,
            crate::save::boxes::get_moves_by_index,
            crate::save::boxes::get_moves_by_box,
            crate::save::boxes::get_ivs_by_index,
            crate::save::boxes::get_ivs,
            crate::save::boxes::get_genders_by_box,
            crate::save::boxes::get_enriched_moves,
            crate::save::boxes::get_xp_by_index,

            crate::save::boxes::get_class_by_index,
            crate::save::boxes::get_classes_by_box,

            crate::save::boxes::grid::get_simple_mon_grid,

            // party
            crate::save::party_legacy::get_enriched_party,
            crate::save::party_legacy::get_pokemon_info,
            crate::save::party_legacy::get_party,

            crate::save::box_edit::set_shiny_by_index,

            run_generator,

        ]
    };
}