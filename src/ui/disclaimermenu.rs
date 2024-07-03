use bevy::prelude::*;

use crate::{assetloader::UiNormalFont, gamestate::GameState};

pub struct DisclaimerMenuPlugin;

impl Plugin for DisclaimerMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DisclaimerMenu), spawn_disclaimer_menu);
        app.add_systems(OnExit(GameState::DisclaimerMenu), despawn_disclaimer_menu);
        app.add_systems(
            Update,
            tos_button_interaction.run_if(in_state(GameState::DisclaimerMenu)),
        );
    }
}

#[derive(Component)]
struct DisclaimerMenu;

#[derive(Component)]
struct AcceptTOSButton {
    pressed: bool,
}

fn spawn_disclaimer_menu(mut commands: Commands, normal_font_handle_res: Res<UiNormalFont>) {
    // Spawn title text
    let spawn_tos_text = |parent: &mut ChildBuilder| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: String::from("声明：本游戏含有成人内容，包含并不仅限于关于性的细节文字和图像描写，和裸露图片。所有游玩本游戏的玩家需要年满18周岁，请确认你已经年满18周岁!"),
                    style: TextStyle {
                        font: normal_font_handle_res.0.clone(),
                        font_size: 30.0,
                        color: Color::GRAY,
                    },
                }],
                justify: JustifyText::Center,
                ..default()
            },
            ..default()
        });
    };

    // Spawn title node
    let spawn_tos_node = |parent: &mut ChildBuilder| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    width: Val::Percent(80.0),
                    height: Val::Percent(50.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::ANTIQUE_WHITE.into(),
                ..default()
            })
            .with_children(spawn_tos_text);
    };

    // Spawn play button
    let spawn_tos_button = |parent: &mut ChildBuilder| {
        // Spawn Play button
        parent
            .spawn((
                AcceptTOSButton { pressed: false },
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Px(50.0),
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
                            value: String::from("我已年满18周岁"),
                            style: TextStyle {
                                font: normal_font_handle_res.0.clone(),
                                font_size: 30.0,
                                color: Color::BLUE,
                            },
                        }],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                });
            });
    };

    commands
        .spawn((
            DisclaimerMenu,
            // Main node
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
        .with_children(spawn_tos_node)
        .with_children(spawn_tos_button);
}

fn despawn_disclaimer_menu(
    mut commands: Commands,
    window_query: Query<Entity, With<DisclaimerMenu>>,
) {
    let entity = window_query.get_single().unwrap();
    commands.entity(entity).despawn_recursive();
}

#[allow(clippy::type_complexity)]
fn tos_button_interaction(
    mut tos_botton_query: Query<
        (&Interaction, &mut BackgroundColor, &mut AcceptTOSButton),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interact, mut backgroundcolor, mut tosbutton) in &mut tos_botton_query {
        match interact {
            Interaction::Pressed => {
                *backgroundcolor = Color::ALICE_BLUE.into();
                tosbutton.pressed = true;
            }
            _ => {
                *backgroundcolor = Color::YELLOW_GREEN.into();
                if tosbutton.pressed {
                    next_state.set(GameState::MainMenu);
                }
            }
        }
    }
}
