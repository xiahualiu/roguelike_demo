// Main bevy crate
use bevy::prelude::*;

// Plugin crates
mod assetloader;
mod gamestate;
mod ui;
mod window;

// Use declarations
use assetloader::AssetLoaderPlugin;
use gamestate::GameStatePlugin;
use ui::disclaimermenu::DisclaimerMenuPlugin;
use ui::mainmenu::MainMenuPlugin;
use window::WindowPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetLoaderPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(DisclaimerMenuPlugin)
            .add_plugins(WindowPlugin)
            .add_plugins(GameStatePlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
