use bevy::prelude::*;

use crate::{
    assetloader::{UiBoldFont, UiNormalFont},
    gamestate::GameState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Spawn
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
        app.add_systems(
            Update,
            play_button_interaction.run_if(in_state(GameState::MainMenu)),
        );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton {
    pressed: bool,
}

fn spawn_main_menu(
    mut commands: Commands,
    bold_font_handle_res: Res<UiBoldFont>,
    normal_font_handle_res: Res<UiNormalFont>,
) {
    // Spawn title text
    let spawn_title_text = |parent: &mut ChildBuilder| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: String::from("游戏测试"),
                    style: TextStyle {
                        font: bold_font_handle_res.0.clone(),
                        font_size: 100.0,
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
    let spawn_title_node = |parent: &mut ChildBuilder| {
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
            .with_children(spawn_title_text);
    };

    // Spawn play button
    let spawn_play_button = |parent: &mut ChildBuilder| {
        // Spawn Play button
        parent
            .spawn((
                PlayButton { pressed: false },
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
                            value: String::from("开始游戏"),
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
            MainMenu,
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
        .with_children(spawn_title_node)
        .with_children(spawn_play_button);
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
    mut next_state: ResMut<NextState<GameState>>,
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
                    next_state.set(GameState::GameRunning);
                }
                playbutton.pressed = false;
            }
        }
    }
}
