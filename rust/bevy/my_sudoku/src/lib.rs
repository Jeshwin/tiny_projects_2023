use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook::*;

mod board;
mod makeui;
mod sudoku;

#[wasm_bindgen]
pub fn start() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::hex(board::THEME.base().hex()).unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (board::WINDOW_WIDTH, board::WINDOW_HEIGHT).into(),
                title: String::from("Sudoku"),
                canvas: Some(String::from("#bevy")),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<board::SudokuBoard>()
        .init_resource::<board::SelectedCell>()
        .add_systems(Startup, board::setup_board)
        .add_systems(Startup, makeui::setup_ui)
        .add_systems(Update, makeui::complete_timer)
        .add_systems(Update, makeui::update_timer_text)
        .add_systems(Update, makeui::tick_timer)
        .add_systems(Update, makeui::update_button_colors)
        .add_systems(Update, makeui::button_system)
        .add_systems(Update, board::draw_board)
        .add_systems(Update, board::handle_mouse_clicks_on_board)
        .add_systems(Update, board::highlight_cells)
        .add_systems(Update, board::cell_input_system)
        .run();
}
