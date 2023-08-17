use bevy::{prelude::*, time::Stopwatch};
use catppuccin::Flavour;
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
    interaction: Interaction,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            coordinates: (0, 0),
            value: 0,
            interaction: Interaction::None,
        }
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
        .add_systems(Startup, setup)
        .add_systems(Update, update_timer_text)
        .add_systems(Update, tick_timer)
        .add_systems(Update, update_button_colors)
        .add_systems(Update, button_update_timer)
        .run();
}

/// create the Sudoku board and the interface
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            let py = (WINDOW_HEIGHT / -2.0)
                + BOARD_PADDING
                + CELL_SIZE / 2.0
                + ((CELL_SIZE + CELL_GAP) * y as f32)
                + (y / 3) as f32 * BOX_GAP;
            let translation_vec = Vec3::new(px, py, 0.);
            let cell_vec = Vec2::new(CELL_SIZE, CELL_SIZE);
            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex(THEME.surface0().hex()).unwrap(),
                            custom_size: Some(cell_vec),
                            ..default()
                        },
                        transform: Transform::from_translation(translation_vec),
                        ..default()
                    },
                    Cell {
                        coordinates: (x, y),
                        value: 0,
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                " ",
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
