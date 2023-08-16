use bevy::prelude::*;
use catppuccin::Flavour::Mocha;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook::*;

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 540.0;
const BOARD_PADDING: f32 = 12.0;
const CELL_GAP: f32 = 5.0;
const BOX_GAP: f32 = 4.0;
const CELL_SIZE: f32 = 52.0;

#[wasm_bindgen]
pub fn start() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::hex(Mocha.base().hex()).unwrap()))
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
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex(Mocha.surface0().hex()).unwrap(),
                    custom_size: Some(cell_vec),
                    ..default()
                },
                transform: Transform::from_translation(translation_vec),
                ..default()
            });
        }
    }

    // ui for resetting or showing solution
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::hex(Mocha.blue().hex()).unwrap().into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Show Solution",
                        TextStyle {
                            font: asset_server.load("fonts/Inter-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::hex(Mocha.text().hex()).unwrap().into(),
                        },
                    ));
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::hex(Mocha.blue().hex()).unwrap().into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset",
                        TextStyle {
                            font: asset_server.load("fonts/Inter-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::hex(Mocha.text().hex()).unwrap().into(),
                        },
                    ));
                });
        });
}
