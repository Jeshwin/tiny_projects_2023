use bevy::prelude::*;
use catppuccin::Flavour;

use crate::sudoku;

pub const WINDOW_WIDTH: f32 = 960.0;
pub const WINDOW_HEIGHT: f32 = 540.0;
pub const BOARD_PADDING: f32 = 12.0;
pub const CELL_GAP: f32 = 5.0;
pub const BOX_GAP: f32 = 4.0;
pub const CELL_SIZE: f32 = 52.0;

pub const THEME: Flavour = Flavour::Macchiato;

#[derive(Component)]
pub struct Cell {
    coordinates: (i32, i32),
}

#[derive(Resource)]
pub struct SudokuBoard {
    pub generated_values: Vec<Vec<u8>>,
    pub current_values: Vec<Vec<u8>>,
    pub solution: Vec<Vec<u8>>,
    pub difficulty: sudoku::Difficulty,
}

impl SudokuBoard {
    pub fn with_difficulty(difficulty: sudoku::Difficulty) -> Self {
        let mut generated_board = sudoku::generate_sudoku(difficulty);
        let values = generated_board.clone();
        sudoku::solve_sudoku(&mut generated_board, true);
        let solution = generated_board;
        SudokuBoard {
            generated_values: values.clone(),
            current_values: values.clone(),
            solution,
            difficulty,
        }
    }
}

impl Default for SudokuBoard {
    fn default() -> Self {
        let mut generated_board = sudoku::generate_sudoku(sudoku::Difficulty::Easy);
        let values = generated_board.clone();
        sudoku::solve_sudoku(&mut generated_board, true);
        let solution = generated_board;
        SudokuBoard {
            generated_values: values.clone(),
            current_values: values.clone(),
            solution,
            difficulty: sudoku::Difficulty::Easy,
        }
    }
}

#[derive(Resource)]
pub struct SelectedCell {
    coordinates: Option<(i32, i32)>,
}

impl Default for SelectedCell {
    fn default() -> Self {
        SelectedCell { coordinates: None }
    }
}

/// draw the Sudoku board
pub fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sudoku_board: Res<SudokuBoard>,
) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // spawn the 81 cells
    for y in 0..9 {
        for x in 0..9 {
            let px = (WINDOW_WIDTH / -2.0)
                + BOARD_PADDING
                + CELL_SIZE / 2.0
                + ((CELL_SIZE + CELL_GAP) * x as f32)
                + (x / 3) as f32 * BOX_GAP;
            let py = ((WINDOW_HEIGHT / -2.0)
                + BOARD_PADDING
                + CELL_SIZE / 2.0
                + ((CELL_SIZE + CELL_GAP) * y as f32)
                + (y / 3) as f32 * BOX_GAP)
                * -1.0;
            let translation_vec = Vec3::new(px, py, 0.);
            let cell_size_vec = Vec2::new(CELL_SIZE, CELL_SIZE);
            let cell_value = sudoku_board.generated_values[y as usize][x as usize];
            let cell_value_string = if cell_value != 0 {
                format!("{}", cell_value)
            } else {
                " ".to_string()
            };
            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex(THEME.surface0().hex()).unwrap(),
                            custom_size: Some(cell_size_vec),
                            ..default()
                        },
                        transform: Transform::from_translation(translation_vec),
                        ..default()
                    },
                    Cell {
                        coordinates: (x, y),
                    },
                ))
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                cell_value_string,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 48.0,
                                    color: Color::hex(THEME.text().hex()).unwrap().into(),
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
        }
    }
}

// draw the board each frame
pub fn draw_board(
    sudoku_board: Res<SudokuBoard>,
    cell_parent: Query<(&Cell, &Children)>,
    mut cell_text_child: Query<&mut Text>,
) {
    for (cell, children) in cell_parent.into_iter() {
        let (cell_x, cell_y) = cell.coordinates;
        for &child in children.iter() {
            let mut cell_text = cell_text_child.get_mut(child).unwrap();
            if sudoku_board.current_values[cell_y as usize][cell_x as usize] == 0 {
                cell_text.sections[0].value = " ".to_string();
            } else {
                cell_text.sections[0].value = format!(
                    "{}",
                    sudoku_board.current_values[cell_y as usize][cell_x as usize]
                );
                if sudoku_board.current_values[cell_y as usize][cell_x as usize]
                    == sudoku_board.generated_values[cell_y as usize][cell_x as usize]
                {
                    cell_text.sections[0].style.color =
                        Color::hex(THEME.text().hex()).unwrap().into();
                } else {
                    if sudoku_board.current_values[cell_y as usize][cell_x as usize]
                        == sudoku_board.solution[cell_y as usize][cell_x as usize]
                    {
                        cell_text.sections[0].style.color =
                            Color::hex(THEME.green().hex()).unwrap().into();
                    } else {
                        cell_text.sections[0].style.color =
                            Color::hex(THEME.red().hex()).unwrap().into();
                    }
                }
            }
        }
    }
}

pub fn handle_mouse_clicks_on_board(
    mouse_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    mut selected_cell: ResMut<SelectedCell>,
) {
    let win = window.get_single().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_x =
            win.cursor_position().unwrap_or(Vec2::new(-1.0, -1.0)).x - (WINDOW_WIDTH / 2.0);
        let cursor_y =
            win.cursor_position().unwrap_or(Vec2::new(-1.0, -1.0)).y - (WINDOW_HEIGHT / 2.0);
        let prev_selected_cell = selected_cell.coordinates.unwrap_or((-1, -1));

        // Check if click intersects a cell
        for y in 0..9 {
            for x in 0..9 {
                let px = (WINDOW_WIDTH / -2.0)
                    + BOARD_PADDING
                    + CELL_SIZE / 2.0
                    + ((CELL_SIZE + CELL_GAP) * x as f32)
                    + (x / 3) as f32 * BOX_GAP;
                let py = (WINDOW_HEIGHT / -2.0)
                    + BOARD_PADDING
                    + CELL_SIZE / 2.0
                    + ((CELL_SIZE + CELL_GAP) * y as f32)
                    + (y / 3) as f32 * BOX_GAP;
                if (cursor_x - px).abs() < CELL_SIZE / 2.0
                    && (cursor_y - py).abs() < CELL_SIZE / 2.0
                {
                    selected_cell.coordinates = Some((x, y));
                    break;
                }
            }
        }

        // If it didn't intersect a cell, set selected_cell to None
        if selected_cell.coordinates.unwrap_or((-1, -1)) == prev_selected_cell {
            selected_cell.coordinates = None;
        }
    }
}

pub fn highlight_cells(
    selected_cell: Res<SelectedCell>,
    mut cells_query: Query<(&mut Sprite, &Cell)>,
) {
    if let Some(selected_cell_coordinates) = selected_cell.coordinates {
        for (mut cell_sprite, cell) in cells_query.iter_mut() {
            if cell.coordinates == selected_cell_coordinates {
                cell_sprite.color = Color::hex(THEME.surface2().hex()).unwrap().into();
            } else if cell.coordinates.0 == selected_cell_coordinates.0
                || cell.coordinates.1 == selected_cell_coordinates.1
                || (cell.coordinates.0 / 3, cell.coordinates.1 / 3)
                    == (
                        selected_cell_coordinates.0 / 3,
                        selected_cell_coordinates.1 / 3,
                    )
            {
                cell_sprite.color = Color::hex(THEME.surface1().hex()).unwrap().into();
            } else {
                cell_sprite.color = Color::hex(THEME.surface0().hex()).unwrap().into();
            }
        }
    } else {
        for (mut cell_sprite, _cell) in cells_query.iter_mut() {
            cell_sprite.color = Color::hex(THEME.surface0().hex()).unwrap().into();
        }
    }
}

pub fn cell_input_system(
    selected_cell: Res<SelectedCell>,
    mut sudoku_board: ResMut<SudokuBoard>,
    kbd: Res<Input<KeyCode>>,
) {
    if let Some((cell_x, cell_y)) = selected_cell.coordinates {
        if sudoku_board.current_values[cell_y as usize][cell_x as usize] == 0
            || sudoku_board.current_values[cell_y as usize][cell_x as usize]
                != sudoku_board.generated_values[cell_y as usize][cell_x as usize]
        {
            if kbd.just_pressed(KeyCode::Key1) || kbd.just_pressed(KeyCode::Numpad1) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 1;
            } else if kbd.just_pressed(KeyCode::Key2) || kbd.just_pressed(KeyCode::Numpad2) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 2;
            } else if kbd.just_pressed(KeyCode::Key3) || kbd.just_pressed(KeyCode::Numpad3) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 3;
            } else if kbd.just_pressed(KeyCode::Key4) || kbd.just_pressed(KeyCode::Numpad4) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 4;
            } else if kbd.just_pressed(KeyCode::Key5) || kbd.just_pressed(KeyCode::Numpad5) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 5;
            } else if kbd.just_pressed(KeyCode::Key6) || kbd.just_pressed(KeyCode::Numpad6) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 6;
            } else if kbd.just_pressed(KeyCode::Key7) || kbd.just_pressed(KeyCode::Numpad7) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 7;
            } else if kbd.just_pressed(KeyCode::Key8) || kbd.just_pressed(KeyCode::Numpad8) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 8;
            } else if kbd.just_pressed(KeyCode::Key9) || kbd.just_pressed(KeyCode::Numpad9) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 9;
            } else if kbd.just_pressed(KeyCode::Key0) || kbd.just_pressed(KeyCode::Numpad0) {
                sudoku_board.current_values[cell_y as usize][cell_x as usize] = 0;
            }
        }
    }
}
