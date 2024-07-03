use bevy::prelude::*;

// Game loading states
#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    AssetLoading,
    DisclaimerMenu,
    MainMenu,
    GameRunning,
    //    PausedMenu,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
