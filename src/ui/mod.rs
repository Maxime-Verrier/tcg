mod main_menu;

use bevy::app::{App, Update};
pub use main_menu::*;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, main_menu_button_system);
}