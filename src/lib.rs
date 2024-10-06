#![allow(clippy::type_complexity)]

mod agent;
mod audio;
mod loading;
mod menu;
mod player_actions;

mod cfg;

use crate::agent::AgentPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player_actions::ActionsPlugin;

use bevy_rapier2d::prelude::*;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
	// During the loading State the LoadingPlugin will load our assets
	#[default]
	Loading,
	// During this State the actual game logic is executed
	Playing,
	// Here the menu is drawn and waiting for player interaction
	Menu,
}

pub struct GamePlugin {
	pxpm: f32,
	debug_render: bool,
}

impl Default for GamePlugin {
	fn default() -> Self {
		Self {
			pxpm: 1.,
			debug_render: true,
		}
	}
}

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.init_state::<GameState>()
			.add_plugins((
				RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(self.pxpm),
				LoadingPlugin,
				MenuPlugin,
				ActionsPlugin,
				InternalAudioPlugin,
				AgentPlugin,
			))
			.insert_resource(RapierConfiguration {
				gravity: Vec2::ZERO,
				timestep_mode: TimestepMode::Fixed {
					dt: 1.0 / 60.0,
					substeps: 1,
				},
				physics_pipeline_active: true,
				query_pipeline_active: true,
				scaled_shape_subdivision: default(),
				force_update_from_transform_changes: false,
			});

		if self.debug_render {
			app.add_plugins(RapierDebugRenderPlugin::default());
		}

		#[cfg(debug_assertions)]
		{
			app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
		}
	}
}
