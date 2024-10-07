pub use bevy::prelude::*;
use bevy::winit::{UpdateMode, WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use card::CardPlugin;
use epithet::net::NetPlugins;
use state::state_plugin;
use ui::ui_plugin;

mod cards;
mod scene;
mod state;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        NetPlugins,
        CardPlugin,
        state_plugin,
        ui_plugin,
    ))
    .run();

    app.insert_resource(WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    });
}
