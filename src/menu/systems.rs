use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::menu::components::*;
use crate::menu::resources::*;
use crate::menu::styles::*;

pub fn fix_menu_first_game(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut timer: ResMut<FixMenuTimer>,
    time: Res<Time>,
) {
    if let Ok(menu_entity) = main_menu_query.get_single() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(menu_entity).despawn_recursive();
            build_main_menu(&mut commands, &asset_server, window_query);
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
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
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
) {
    build_main_menu(&mut commands, &asset_server, window_query);
}

fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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
                        texture: asset_server.load("sprites/Play-Button.png"),
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
                        texture: asset_server.load("sprites/Quit-Button.png"),
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
