use bevy::{
    app::{App, Update},
    input::ButtonInput,
    prelude::{
        in_state, resource_exists, AppExtStates, IntoSystemConfigs, KeyCode, NextState, OnEnter,
        OnExit, Res, ResMut, States,
    },
};
use epithet::{
    net::{NetState, UserInfo},
    utils::clean_scene,
};

use crate::{
    scene::{create_dev_room_core_scene, create_dev_room_scene},
    ui::create_main_menu,
};

#[derive(States, Default, Hash, Clone, Eq, PartialEq, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

pub fn state_plugin(app: &mut App) {
    app.init_state::<AppState>();

    app.add_systems(Update, leave_game_state); //TODO make it before ServerSet::Send ?

    // AppStates transitions
    app.add_systems(OnEnter(AppState::MainMenu), create_main_menu);
    app.add_systems(OnExit(AppState::MainMenu), clean_scene);
    app.add_systems(
        OnEnter(AppState::Game),
        (
            create_dev_room_core_scene,
            create_dev_room_scene.run_if(in_state(NetState::Server)),
        ),
    );
    app.add_systems(OnExit(AppState::Game), clean_scene);
}

pub(crate) fn leave_game_state(
    keys: Res<ButtonInput<KeyCode>>,
    mut states: ResMut<NextState<AppState>>,
    mut net_states: ResMut<NextState<NetState>>,
) {
    if keys.pressed(KeyCode::Escape) {
        states.set(AppState::MainMenu);
        net_states.set(NetState::None);
    }
}
