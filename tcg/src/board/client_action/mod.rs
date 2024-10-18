mod action;
mod summon;

#[cfg(all(feature = "render", feature = "client"))]
pub(crate) use cfg_client_action::*;

#[cfg(all(feature = "render", feature = "client"))]
mod cfg_client_action {
    pub use super::action::*;
    pub use super::summon::*;

    use super::summon::{summon_action_execute, summon_action_finish};
    use bevy::prelude::*;

    pub(crate) fn board_action_plugin(app: &mut bevy::app::App) {
        use crate::state::AppState;

        app.insert_resource(ClientActionState::default());

        app.add_systems(OnEnter(AppState::Game), action_state_setup);
        app.add_systems(OnExit(AppState::Game), action_state_cleanup);

        app.observe(summon_action_execute); //TODO only attach to a target, on  self agent ?
        app.observe(summon_action_finish);
    }

    //MAYBE change how resource is inserted ? this will cause an issue if the player is on multiple boards, i don't think it will happen
    //MAYBE Also maybe just insert it when the player join a board, but honestly it's negligable memory save
    fn action_state_setup(mut commands: Commands) {
        commands.insert_resource(ClientActionState::default());
    }

    fn action_state_cleanup(mut commands: Commands) {
        commands.remove_resource::<ClientActionState>();
    }
}
