pub use bevy::prelude::*;
use bevy::{
    window::PrimaryWindow,
    winit::{UpdateMode, WinitSettings},
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui, DefaultInspectorConfigPlugin,
};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_replicon::prelude::AppRuleExt;
use board::board_plugin;
use card::card_plugin;
use card_sim::CardSimPlugin;
use epithet::{net::NetPlugins, units::UnitPlugin};
use state::state_plugin;
use ui::ui_plugin;

mod board;
mod card;
mod scene;
mod state;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        DefaultInspectorConfigPlugin,
        EguiPlugin,
        DefaultPickingPlugins,
        NetPlugins,
        UnitPlugin,
        CardSimPlugin,
        state_plugin,
        ui_plugin,
        card_plugin,
        board_plugin
    ))
    .add_systems(Update, inspector_ui);

    app.insert_resource(WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    });

    app.replicate::<Name>();
    app.replicate::<GlobalTransform>();
    app.replicate::<Transform>();

    app.run();
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::CollapsingHeader::new("Game Debug").show(ui, |ui| {
                if ui.add(egui::Button::new("Add Card")).clicked() {
                    //TODO
                }
            });
            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
            });
        });
    });
}
