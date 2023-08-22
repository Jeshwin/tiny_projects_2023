use crate::{board, sudoku};
use bevy::{prelude::*, time::Stopwatch};
use catppuccin::Flavour;

const BOARD_PADDING: f32 = 12.0;

const THEME: Flavour = Flavour::Macchiato;

#[derive(Component)]
pub struct SudokuTimerComponent {
    time: Stopwatch,
}

// tag component for each button
#[derive(Component)]
pub struct ShowSolution;

#[derive(Component)]
pub struct ResetBoard;

#[derive(Component)]
pub struct NewBoard;

// draw the ui
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn update_timer_text(mut timer_query: Query<(&mut Text, &SudokuTimerComponent)>) {
    let (mut timer_text, sudoku_timer) = timer_query.single_mut();
    // convert elapsed seconds into a timer format
    let seconds = sudoku_timer.time.elapsed_secs();
    let minutes = (seconds / 60.0).floor() as i32;
    let remaining_seconds = (seconds % 60.0).round() as i32;
    timer_text.sections[0].value = format!("{}:{:02}", minutes, remaining_seconds);
}

// stopwatch has to be ticked to progress
pub fn tick_timer(mut timer_query: Query<&mut SudokuTimerComponent>, time: Res<Time>) {
    let mut sudoku_timer = timer_query.single_mut();
    sudoku_timer.time.tick(time.delta());
}

pub fn update_button_colors(
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

pub fn button_system(
    mut timer_query: Query<&mut SudokuTimerComponent>,
    show_solution_button_query: Query<&Interaction, (Changed<Interaction>, With<ShowSolution>)>,
    reset_board_button_query: Query<&Interaction, (Changed<Interaction>, With<ResetBoard>)>,
    new_board_button_query: Query<&Interaction, (Changed<Interaction>, With<NewBoard>)>,
    mut sudoku_board: ResMut<board::SudokuBoard>,
) {
    let mut sudoku_timer = timer_query.single_mut();

    if let Ok(&show_solution_interaction) = show_solution_button_query.get_single() {
        if show_solution_interaction == Interaction::Pressed {
            sudoku_board.current_values = sudoku_board.solution.clone();
            sudoku_timer.time.pause();
        }
    }
    if let Ok(&reset_board_interaction) = reset_board_button_query.get_single() {
        if reset_board_interaction == Interaction::Pressed {
            sudoku_board.current_values = sudoku_board.generated_values.clone();
            sudoku_timer.time.unpause();
            sudoku_timer.time.reset();
        }
    }
    if let Ok(&new_board_interaction) = new_board_button_query.get_single() {
        if new_board_interaction == Interaction::Pressed {
            if sudoku_board.difficulty == sudoku::Difficulty::Easy {
                *sudoku_board = board::SudokuBoard::with_difficulty(sudoku::Difficulty::Medium);
            } else {
                *sudoku_board = board::SudokuBoard::with_difficulty(sudoku::Difficulty::Hard);
            }
            sudoku_timer.time.unpause();
            sudoku_timer.time.reset();
        }
    }
}

pub fn complete_timer(
    mut timer_query: Query<(&mut Text, &mut SudokuTimerComponent)>,
    sudoku_board: ResMut<board::SudokuBoard>,
) {
    let (mut timer_text, mut sudoku_timer) = timer_query.single_mut();

    if sudoku_board.current_values == sudoku_board.solution {
        sudoku_timer.time.pause();
        timer_text.sections[0].style.color = Color::hex(THEME.green().hex()).unwrap().into();
    } else {
        timer_text.sections[0].style.color = Color::hex(THEME.subtext0().hex()).unwrap().into();
    }
}
