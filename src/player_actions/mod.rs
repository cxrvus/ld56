use bevy::prelude::*;

use crate::player_actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<PlayerActions>().add_systems(
			Update,
			set_movement_actions.run_if(in_state(GameState::Playing)),
		);
	}
}

#[derive(Default, Resource)]
pub struct PlayerActions {
	pub player_direction: Option<Vec2>,
}

pub fn set_movement_actions(
	mut actions: ResMut<PlayerActions>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	let player_movement = Vec2::new(
		get_movement(GameControl::Right, &keyboard_input)
			- get_movement(GameControl::Left, &keyboard_input),
		get_movement(GameControl::Up, &keyboard_input)
			- get_movement(GameControl::Down, &keyboard_input),
	);

	if player_movement != Vec2::ZERO {
		actions.player_direction = Some(player_movement.normalize());
	} else {
		actions.player_direction = None;
	}
}
