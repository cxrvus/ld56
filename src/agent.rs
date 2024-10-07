use crate::cfg::{BASE_SCALE, BASE_SPEED, SPRITE_SCALE};
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
				translation: Vec3::new(BASE_SCALE * 2., 0., 1.),
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

	commands
		.spawn(Sprite {
			color: Color::BLACK,
			custom_size: Some(Vec2::new(SPRITE_SCALE, SPRITE_SCALE)),
			..default()
		})
		.insert(Collider::cuboid(BASE_SCALE, BASE_SCALE));
}

#[derive(Component)]
pub struct _Player;

#[derive(Component, Default)]
pub struct Agent {
	pub direction: Vec2,
	pub speed: f32,
	pub rotation: Quat,
}

fn set_agent_movements(
	mut agents: Query<&mut Agent>, // todo: without Player
	actions: Res<PlayerActions>,
) {
	for mut agent in agents.iter_mut() {
		let direction = actions.player_direction.unwrap_or_default().normalize(); // todo: agent AI movement instead of player movement

		let moving = direction.length_squared() > 0.0;
		if moving {
			agent.direction = direction;
			agent.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.).normalize());
			agent.speed = BASE_SPEED;
		} else {
			agent.direction = Vec2::ZERO;
			agent.speed = 0.;
			// todo: else default
		}
	}
}

fn move_agent(mut agents: Query<(&Agent, &mut Transform, &mut Velocity)>, time: Res<Time>) {
	for (agent, mut transform, mut velocity) in &mut agents {
		velocity.linvel = agent.direction * agent.speed * time.delta_seconds();
		transform.rotation = agent.rotation;
	}
}
