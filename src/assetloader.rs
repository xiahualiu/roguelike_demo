use bevy::asset::{RecursiveDependencyLoadState, UntypedAssetId};
use bevy::prelude::*;

use crate::gamestate::GameState;

// Game loading states
#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone)]
pub enum AssetLoadingState {
    #[default]
    NotLoaded,
    Loading,
    DoneLoading,
    FailedLoading,
}

// The font resources used by menu
#[derive(Resource)]
pub struct UiNormalFont(pub Handle<Font>);

#[derive(Resource)]
pub struct UiBoldFont(pub Handle<Font>);

// The bevy logo image
#[derive(Resource)]
pub struct BevyLogoImage(pub Handle<Image>);

// AssetId vector to check loading status
#[derive(Resource)]
struct LoadAssetIdVec(Vec<UntypedAssetId>);

// Store the loading status data
#[derive(Resource)]
struct LoadStatus {
    total: u64,
    loaded: u64,
}

#[derive(Component)]
struct AssetLoadingMenu;

#[derive(Component)]
struct ProgressBar;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>()
            .insert_resource(LoadAssetIdVec(Vec::new()))
            .insert_resource(LoadStatus {
                total: 0,
                loaded: 0,
            })
            .add_systems(
                OnEnter(GameState::AssetLoading),
                (load_assets, spawn_loading_menu).chain(),
            )
            .add_systems(
                Update,
                (poll_all_load_status, update_load_bar)
                    .chain()
                    .run_if(in_state(AssetLoadingState::Loading)),
            )
            .add_systems(
                OnEnter(AssetLoadingState::DoneLoading),
                despawn_loading_menu,
            );
    }
}

/// `load_assets` Only run once entering the Loading state  
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_ids: ResMut<LoadAssetIdVec>,
    mut load_status: ResMut<LoadStatus>,
    mut next_load_state: ResMut<NextState<AssetLoadingState>>,
) {
    // Set the total number of assets
    load_status.total = 3;

    // Load han sans normal font
    let handle_font = asset_server.load("fonts/NotoSansCJKsc-Regular.otf");
    asset_ids.0.push(handle_font.clone().untyped().id());
    commands.insert_resource(UiNormalFont(handle_font));
    // Load han sans bold font
    let handle_font = asset_server.load("fonts/NotoSansCJKsc-Bold.otf");
    asset_ids.0.push(handle_font.clone().untyped().id());
    commands.insert_resource(UiBoldFont(handle_font));
    // Load bevy icon
    let bevy_logo_handle = asset_server.load("textures/bevy.png");
    asset_ids.0.push(bevy_logo_handle.clone().untyped().id());
    commands.insert_resource(BevyLogoImage(bevy_logo_handle));

    // Move state to Loading
    next_load_state.set(AssetLoadingState::Loading);
}

/// System that polls all load status and moves states
fn poll_all_load_status(
    mut next_load_state: ResMut<NextState<AssetLoadingState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    asset_vec: Res<LoadAssetIdVec>,
    mut load_status: ResMut<LoadStatus>,
) {
    // Clear loaded count
    load_status.loaded = 0;
    for asset_id in asset_vec.0.iter() {
        match asset_server
            .get_recursive_dependency_load_state(*asset_id)
            .unwrap()
        {
            RecursiveDependencyLoadState::Failed => {
                next_load_state.set(AssetLoadingState::FailedLoading);
            }
            RecursiveDependencyLoadState::Loaded => {
                load_status.loaded += 1;
            }
            _ => {}
        }
    }
    if load_status.loaded == load_status.total {
        next_load_state.set(AssetLoadingState::DoneLoading);
        next_game_state.set(GameState::DisclaimerMenu);
    }
}

fn update_load_bar(
    mut bar_query: Query<&mut Style, With<ProgressBar>>,
    load_status: Res<LoadStatus>,
) {
    let percent = load_status.loaded as f32 / load_status.total as f32;
    bar_query.get_single_mut().unwrap().width = Val::Percent(100.0 * percent);
}

fn spawn_loading_menu(mut commands: Commands, bevy_logo_res: Res<BevyLogoImage>) {
    // Spawn Bevy Icon Image and Text
    let spawn_bevy_icon_image = |parent: &mut ChildBuilder| {
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                right: Val::Px(10.0),
                ..default()
            },
            image: UiImage {
                texture: bevy_logo_res.0.clone(),
                ..default()
            },
            ..default()
        });
    };

    let spawn_bevy_icon_text = |parent: &mut ChildBuilder| {
        parent.spawn(TextBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: String::from("Powered by Bevy Engine."),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::GRAY,
                        ..default()
                    },
                }],
                justify: JustifyText::Left,
                ..default()
            },
            ..default()
        });
    };

    // Spawn Bevy Icon Node
    let spawn_bevy_icon_node = |parent: &mut ChildBuilder| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::ANTIQUE_WHITE.into(),
                ..default()
            })
            .with_children(spawn_bevy_icon_image)
            .with_children(spawn_bevy_icon_text);
    };

    // Spawn progress bar
    let spawn_progress_bar = |parent: &mut ChildBuilder| {
        parent.spawn((
            ProgressBar,
            NodeBundle {
                style: Style {
                    width: Val::Px(0.0),
                    height: Val::Px(50.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
        ));
    };

    // Spawn progress bar node
    let spawn_progress_bar_node = |parent: &mut ChildBuilder| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    width: Val::Percent(80.0),
                    height: Val::Px(50.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::WHITE.into(),
                ..default()
            })
            .with_children(spawn_progress_bar);
    };

    commands
        .spawn((
            AssetLoadingMenu,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(50.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::ANTIQUE_WHITE.into(),
                ..default()
            },
        ))
        .with_children(spawn_bevy_icon_node)
        .with_children(spawn_progress_bar_node);
}

fn despawn_loading_menu(mut commands: Commands, menu_query: Query<Entity, With<AssetLoadingMenu>>) {
    let entity = menu_query.get_single().unwrap();
    commands.entity(entity).despawn_recursive();
}
