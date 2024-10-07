use crate::GameState;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::LevelGeneration), generate_map);
	}
}

fn generate_map(mut next_state: ResMut<NextState<GameState>>, mut _commands: Commands) {
	// todo: implement
	next_state.set(GameState::Playing);
}
