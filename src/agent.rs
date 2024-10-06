use crate::cfg::{self, SPRITE_SCALE};
use crate::loading::SpriteAssets;
use crate::player_actions::PlayerActions;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct AgentPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for AgentPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_agent)
			.add_systems(
				Update,
				(set_agent_movements, move_agent).run_if(in_state(GameState::Playing)),
			);
	}
}

fn spawn_agent(mut commands: Commands, sprites: Res<SpriteAssets>) {
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
		.insert(Agent::default())
		.insert(RigidBody::Dynamic)
		.insert(Velocity::default())
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(Collider::ball(10.));
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Agent {
	pub movement: Vec2,
	pub rotation: Quat,
}

fn set_agent_movements(
	mut agents: Query<&mut Agent>, // todo: without Player
	actions: Res<PlayerActions>,
	time: Res<Time>,
) {
	for mut agent in agents.iter_mut() {
		let movement = actions.player_movement.unwrap_or_default(); // todo: agent AI movement instead of player movement

		// todo: make speed part of agent struct
		let speed = cfg::BASE_SPEED;
		let movement = movement * speed * time.delta_seconds();

		let moving = movement.length_squared() > 0.0;
		if moving {
			agent.movement = movement;
			agent.rotation = Quat::from_rotation_arc(Vec3::Y, movement.extend(0.).normalize());
		} else {
			agent.movement = Vec2::ZERO;
		}
	}
}

fn move_agent(mut agents: Query<(&Agent, &mut Transform, &mut Velocity)>) {
	for (agent, mut transform, mut velocity) in &mut agents {
		velocity.linvel = agent.movement;
		transform.rotation = agent.rotation;
	}
}
