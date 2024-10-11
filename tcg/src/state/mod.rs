use bevy::{
    app::{App, Update},
    input::ButtonInput,
    prelude::{
        AppExtStates, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Res, ResMut, States,
    },
};
use epithet::{
    net::NetState,
    utils::{clean_scene, not_in_state},
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
            // Not using singeplayer_or_server because as net_state is not set yet the client is not connected yet
            // TODO maybe change how we check as this could cause issue maybe ? only if we use a condition that can be true while client is not connected and we depend on it
            // TODO rethink run conditions maybe, or just add it to systems that require it
            create_dev_room_scene
                .run_if(not_in_state(NetState::Client))
                .after(create_dev_room_core_scene),
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
