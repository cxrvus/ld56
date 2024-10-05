use crate::actions::Actions;
use crate::loading::{SpriteAssets, TextureAssets};
use crate::pixels::SPRITE_SCALE;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_player)
			.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
	}
}

fn spawn_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
	commands
		.spawn(SpriteBundle {
			texture: sprites.louse.clone(),
			transform: Transform {
				translation: Vec3::new(0., 0., 1.),
				scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
				..default()
			},
			..default()
		})
		.insert(Player);
}

fn move_player(
	time: Res<Time>,
	actions: Res<Actions>,
	mut player_query: Query<&mut Transform, With<Player>>,
) {
	if actions.player_movement.is_none() {
		return;
	}

	let speed = 300.;

	let movement = Vec3::new(
		actions.player_movement.unwrap().x * speed * time.delta_seconds(),
		actions.player_movement.unwrap().y * speed * time.delta_seconds(),
		0.,
	);

	let rotation = if movement.length_squared() > 0.0 {
		Quat::from_rotation_arc(Vec3::Y, movement.normalize())
	} else {
		Quat::IDENTITY
	};

	for mut player_transform in &mut player_query {
		player_transform.translation += movement;
		player_transform.rotation = rotation;
	}
}
