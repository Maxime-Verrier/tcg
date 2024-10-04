pub use bevy::prelude::*;
use bevy::winit::{UpdateMode, WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use epithet::net::NetPlugins;
use state::state_plugin;
use ui::ui_plugin;

mod state;
mod ui;
mod scene;
mod cards;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), NetPlugins, state_plugin, ui_plugin)).run();

    app.insert_resource(WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    });
}