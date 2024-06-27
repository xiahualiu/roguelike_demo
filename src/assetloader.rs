use bevy::asset::{AssetMetaCheck, RecursiveDependencyLoadState, UntypedAssetId};
use bevy::prelude::*;

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
pub struct UiFont(pub Handle<Font>);

// AssetId vector to check loading status
#[derive(Resource)]
struct LoadAssetIdVec(Vec<UntypedAssetId>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>()
            .insert_resource(LoadAssetIdVec(Vec::new()))
            .insert_resource(AssetMetaCheck::Never)
            .add_systems(OnEnter(AssetLoadingState::Loading), load_assets)
            .add_systems(
                Update,
                poll_all_load_status.run_if(in_state(AssetLoadingState::Loading)),
            )
            .add_systems(Startup, setup_state);
    }
}

/// `load_assets` Only run once entering the Loading state  
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_ids: ResMut<LoadAssetIdVec>,
) {
    let handle_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    asset_ids.0.push(handle_font.clone().untyped().id());
    commands.insert_resource(UiFont(handle_font));
}

/// System that polls all load status and moves the AssetLoadingState
fn poll_all_load_status(
    mut next_load_state: ResMut<NextState<AssetLoadingState>>,
    asset_server: Res<AssetServer>,
    asset_vec: Res<LoadAssetIdVec>,
) {
    match check_all_load_status(&asset_server, &asset_vec) {
        RecursiveDependencyLoadState::NotLoaded => {}
        RecursiveDependencyLoadState::Loading => {}
        RecursiveDependencyLoadState::Failed => {
            next_load_state.set(AssetLoadingState::FailedLoading);
        }
        RecursiveDependencyLoadState::Loaded => {
            next_load_state.set(AssetLoadingState::DoneLoading);
        }
    }
}

/// Loop over all asset ids in the AssetIds and return based on the
/// loading results.
fn check_all_load_status(
    asset_server: &Res<AssetServer>,
    asset_vec: &Res<LoadAssetIdVec>,
) -> RecursiveDependencyLoadState {
    for asset_id in asset_vec.0.iter() {
        match asset_server
            .get_recursive_dependency_load_state(*asset_id)
            .unwrap()
        {
            RecursiveDependencyLoadState::NotLoaded => {
                return RecursiveDependencyLoadState::NotLoaded;
            }
            RecursiveDependencyLoadState::Loading => {
                return RecursiveDependencyLoadState::Loading;
            }
            RecursiveDependencyLoadState::Loaded => continue,
            RecursiveDependencyLoadState::Failed => {
                return RecursiveDependencyLoadState::Failed;
            }
        };
    }
    RecursiveDependencyLoadState::Loaded
}

/// Debug use
fn setup_state(mut next_load_state: ResMut<NextState<AssetLoadingState>>) {
    next_load_state.set(AssetLoadingState::Loading);
}
