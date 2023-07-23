use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::menu::components::*;
use crate::menu::resources::*;
use crate::menu::styles::*;

pub fn setup_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

    commands.spawn((
        ImageBundle {
            image: asset_server.load("cursor/cursor.png").into(),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(15),
            transform: Transform::from_translation(cursor_spawn),
            ..default()
        },
        GameCursor {}
    ));
}

pub fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();
        img_style.left = Val::Px(position.x - 2.0);
        img_style.bottom = Val::Px((window.height() - position.y) -24.0);
    }
}


pub fn fix_menu_first_game(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut timer: ResMut<FixMenuTimer>,
    time: Res<Time>,
    game_state_const: Res<State<GameState>>,
) {
    if let Ok(menu_entity) = main_menu_query.get_single() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(menu_entity).despawn_recursive();
            build_main_menu(&mut commands, &asset_server, window_query, game_state_const);
        }
    }
}

pub fn pause_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    game_state_const: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match *game_state_const.get() {
            GameState::Game => {
                game_state.set(GameState::Paused);
            }
            GameState::Paused => {
                game_state.set(GameState::Game);
            }
            GameState::Menu => {}
        }
    }
}

pub fn interact_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    game_state_const: Res<State<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();

                match *game_state_const.get() {
                    GameState::Menu => {
                        game_state.set(GameState::Game);
                    }
                    GameState::Paused => {
                        game_state.set(GameState::Game);
                    }
                    GameState::Game => {}
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    game_state_const: Res<State<GameState>>,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();

                match *game_state_const.get() {
                    GameState::Menu => {
                        game_state.set(GameState::Game);
                        app_exit_event_writer.send(AppExit);
                    }
                    GameState::Paused => {
                        if let Ok(main_menu_entity) = main_menu_query.get_single() {
                            commands.entity(main_menu_entity).despawn();
                        }
                        game_state.set(GameState::Menu);
                    }
                    GameState::Game => {}
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_options_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OptionsButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state_const: Res<State<GameState>>,
) {
    build_main_menu(&mut commands, &asset_server, window_query, game_state_const);
}

fn play_or_resume(game_state_const: &Res<State<GameState>>) -> &'static str {
    match *game_state_const.get() {
        GameState::Menu => {
            return "sprites/Play-Button.png";
        }
        GameState::Paused => {
            return "sprites/Resume-Button.png";
        }
        GameState::Game => {
            return "sprites/Play-Button.png";
        }
    }
}

fn quit_or_main_menu(game_state_const: &Res<State<GameState>>) -> &'static str {
    match *game_state_const.get() {
        GameState::Menu => {
            return "sprites/Quit-Button.png";
        }
        GameState::Paused => {
            return "sprites/Menu-Button.png";
        }
        GameState::Game => {
            return "sprites/Quit-Button.png";
        }
    }
}

fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state_const: Res<State<GameState>>,
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: main_menu_style(window_query),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: title_style(),
                    ..default()
                })
                .with_children(|parent| {
                    // Shadow Text
                    for &offset in &[
                        Vec2::new(-1.0, -1.0),
                        Vec2::new(1.0, -1.0),
                        Vec2::new(-1.0, 1.0),
                        Vec2::new(1.0, 1.0),
                    ] {
                        parent.spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(offset.x),
                                top: Val::Px(offset.y),
                                ..Default::default()
                            },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Shoyu",
                                    get_shadow_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    }
                    // Main Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Shoyu",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Play Button ===
            parent.spawn((
                ButtonBundle {
                    style: button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    image: UiImage {
                        texture: asset_server.load(play_or_resume(&game_state_const)),
                        ..default()
                    },
                    ..default()
                },
                PlayButton {},
            ));
            // === Options Button ===
            parent.spawn((
                ButtonBundle {
                    style: button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    image: UiImage {
                        texture: asset_server.load("sprites/Options-Button.png"),
                        ..default()
                    },
                    ..default()
                },
                OptionsButton {},
            ));
            // === Quit Button ===
            parent.spawn((
                ButtonBundle {
                    style: button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    image: UiImage {
                        texture: asset_server.load(quit_or_main_menu(&game_state_const)),
                        ..default()
                    },
                    ..default()
                },
                QuitButton {},
            ));
        })
        .id();
    return main_menu_entity;
}
