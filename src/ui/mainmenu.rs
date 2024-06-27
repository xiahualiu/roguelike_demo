use bevy::prelude::*;

use crate::assetloader::{AssetLoadingState, UiFont};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Spawn
        app.add_systems(OnEnter(AssetLoadingState::DoneLoading), spawn_main_menu);
        app.add_systems(OnExit(AssetLoadingState::DoneLoading), despawn_main_menu);
        app.add_systems(
            Update,
            play_button_interaction.run_if(in_state(AssetLoadingState::DoneLoading)),
        );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton {
    pressed: bool,
}

fn spawn_main_menu(mut commands: Commands, font_handle_res: Res<UiFont>) {
    // Main node
    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::ANTIQUE_WHITE.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Spawn
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    visibility: Visibility::Visible,
                    background_color: Color::ANTIQUE_WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Breakout"),
                                style: TextStyle {
                                    font: font_handle_res.0.clone(),
                                    font_size: 80.0,
                                    color: Color::GRAY,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Spawn Play button
            parent
                .spawn((
                    PlayButton { pressed: false },
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::YELLOW_GREEN.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Play"),
                                style: TextStyle {
                                    font: font_handle_res.0.clone(),
                                    font_size: 40.0,
                                    color: Color::BLUE,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn despawn_main_menu(mut commands: Commands, window_query: Query<Entity, With<MainMenu>>) {
    let entity = window_query.get_single().unwrap();
    commands.entity(entity).despawn_recursive();
}

#[allow(clippy::type_complexity)]
fn play_button_interaction(
    mut play_botton_query: Query<
        (&Interaction, &mut BackgroundColor, &mut PlayButton),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AssetLoadingState>>,
) {
    for (interact, mut backgroundcolor, mut playbutton) in &mut play_botton_query {
        match interact {
            Interaction::Pressed => {
                *backgroundcolor = Color::ALICE_BLUE.into();
                playbutton.pressed = true;
            }
            _ => {
                *backgroundcolor = Color::YELLOW_GREEN.into();
                if playbutton.pressed {
                    next_state.set(AssetLoadingState::DoneLoading);
                }
            }
        }
    }
}
