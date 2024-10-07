use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::DefaultPlugins;

use licemind::GamePlugin;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	let first = args.get(1);
	if let Some(first) = first {
		match first.as_str() {
			"levelgen" => licemind::level::convert_levels(),
			_ => eprintln!("invalid sub-command: '{first}'"),
		};
	} else {
		game_main();
	};
}

fn game_main() {
	App::new()
		.insert_resource(Msaa::Off)
		.insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1)))
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "LICEMIND".to_string(),
						// Bind to canvas included in `index.html`
						canvas: Some("#bevy".to_owned()),
						fit_canvas_to_parent: true,
						// Tells wasm not to override default event handling, like F5 and Ctrl+R
						prevent_default_event_handling: false,
						..default()
					}),
					..default()
				})
				.set(AssetPlugin {
					meta_check: AssetMetaCheck::Never,
					..default()
				})
				.set(ImagePlugin::default_nearest()),
		)
		.add_plugins(GamePlugin::default())
		.run();
}
