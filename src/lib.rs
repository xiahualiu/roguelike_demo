// Main bevy crate
use bevy::prelude::*;

// Plugin crates
mod assetloader;
mod ui;
mod window;

// Use declarations
use assetloader::{AssetLoaderPlugin, AssetLoadingState};
use ui::mainmenu::MainMenuPlugin;
use window::WindowPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>()
            .add_plugins(AssetLoaderPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(WindowPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}