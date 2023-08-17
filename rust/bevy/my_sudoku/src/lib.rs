use bevy::{prelude::*, time::Stopwatch};
use catppuccin::Flavour;
use sudoku::Sudoku;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook::*;

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 540.0;
const BOARD_PADDING: f32 = 12.0;
const CELL_GAP: f32 = 5.0;
const BOX_GAP: f32 = 4.0;
const CELL_SIZE: f32 = 52.0;

const THEME: Flavour = Flavour::Frappe;

#[derive(Component)]
struct SudokuTimerComponent {
    time: Stopwatch,
}

#[derive(Component)]
struct Cell {
    coordinates: (i32, i32),
    value: u8,
}

#[derive(Resource)]
struct SudokuBoard {
    values: Vec<Vec<u8>>,
}

impl Default for SudokuBoard {
    fn default() -> Self {
        let generated_board = Sudoku::generate_unique().to_bytes();
        let mut values = vec![];

        for r in 0..9 {
            let mut values_row = vec![];
            for c in 0..9 {
                values_row.push(generated_board[r * 9 + c]);
            }
            values.push(values_row);
        }

        SudokuBoard { values }
    }
}

#[derive(Resource)]
struct SelectedCell {
    coordinates: Option<(i32, i32)>,
}

impl Default for SelectedCell {
    fn default() -> Self {
        SelectedCell { coordinates: None }
    }
}

// tag component for each button
#[derive(Component)]
struct ShowSolution;

#[derive(Component)]
struct ResetBoard;

#[derive(Component)]
struct NewBoard;

#[wasm_bindgen]
pub fn start() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::hex(THEME.base().hex()).unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: String::from("Sudoku"),
                canvas: Some(String::from("#bevy")),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SudokuBoard>()
        .init_resource::<SelectedCell>()
        .add_systems(Startup, setup_board)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, update_timer_text)
        .add_systems(Update, tick_timer)
        .add_systems(Update, update_button_colors)
        .add_systems(Update, button_update_timer)
        .add_systems(Update, handle_mouse_clicks)
        .add_systems(Update, highlight_cells)
        .run();
}

/// draw the Sudoku board
fn setup_board(
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
            let cell_value = sudoku_board.values[y as usize][x as usize];
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
                        value: cell_value,
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

// draw the ui
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui buttons and timer
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                height: Val::Percent(100.0),
                width: Val::Px(420.0 - 2.0 * BOARD_PADDING),
                right: Val::Px(BOARD_PADDING),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // timer
            parent.spawn((
                TextBundle::from_section(
                    "0:00",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 65.0,
                        color: Color::hex(THEME.subtext0().hex()).unwrap().into(),
                    },
                ),
                SudokuTimerComponent {
                    time: Stopwatch::new(),
                },
            ));

            // show solution button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::hex(THEME.overlay0().hex()).unwrap().into(),
                        ..default()
                    },
                    ShowSolution,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Show Solution",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::hex(THEME.text().hex()).unwrap().into(),
                        },
                    ));
                });

            // reset board button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::hex(THEME.overlay0().hex()).unwrap().into(),
                        ..default()
                    },
                    ResetBoard,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset Board",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::hex(THEME.text().hex()).unwrap().into(),
                        },
                    ));
                });

            // new board button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::hex(THEME.overlay0().hex()).unwrap().into(),
                        ..default()
                    },
                    NewBoard,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New Board",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::hex(THEME.text().hex()).unwrap().into(),
                        },
                    ));
                });
        });
}

fn update_timer_text(mut timer_query: Query<(&mut Text, &SudokuTimerComponent)>) {
    let (mut timer_text, sudoku_timer) = timer_query.single_mut();
    // convert elapsed seconds into a timer format
    let seconds = sudoku_timer.time.elapsed_secs();
    let minutes = (seconds / 60.0).floor() as i32;
    let remaining_seconds = (seconds % 60.0).round() as i32;
    timer_text.sections[0].value = format!("{}:{:02}", minutes, remaining_seconds);
}

// stopwatch has to be ticked to progress
fn tick_timer(mut timer_query: Query<&mut SudokuTimerComponent>, time: Res<Time>) {
    let mut sudoku_timer = timer_query.single_mut();
    sudoku_timer.time.tick(time.delta());
}

fn update_button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::hex(THEME.overlay2().hex()).unwrap().into();
            }
            Interaction::Hovered => {
                *color = Color::hex(THEME.overlay1().hex()).unwrap().into();
            }
            Interaction::None => {
                *color = Color::hex(THEME.overlay0().hex()).unwrap().into();
            }
        }
    }
}

fn button_update_timer(
    mut timer_query: Query<&mut SudokuTimerComponent>,
    show_solution_button_query: Query<&Interaction, With<ShowSolution>>,
    reset_board_button_query: Query<&Interaction, With<ResetBoard>>,
    new_board_button_query: Query<&Interaction, With<NewBoard>>,
) {
    let mut sudoku_timer = timer_query.single_mut();

    let &show_solution_interaction = show_solution_button_query.single();
    if show_solution_interaction == Interaction::Pressed {
        sudoku_timer.time.pause();
    }
    let &reset_board_interaction = reset_board_button_query.single();
    if reset_board_interaction == Interaction::Pressed {
        sudoku_timer.time.unpause();
        sudoku_timer.time.reset();
    }
    let &new_board_interaction = new_board_button_query.single();
    if new_board_interaction == Interaction::Pressed {
        sudoku_timer.time.unpause();
        sudoku_timer.time.reset();
    }
}

fn handle_mouse_clicks(
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
        println!("click at ({cursor_x:.0}, {cursor_y:.0})");

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

fn highlight_cells(selected_cell: Res<SelectedCell>, mut cells_query: Query<(&mut Sprite, &Cell)>) {
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
