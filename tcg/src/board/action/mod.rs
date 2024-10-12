mod action;
mod summon;

pub use action::*;
pub use summon::*;

use crate::state::AppState;

pub(crate) fn board_action_plugin(app: &mut bevy::app::App) {
    app.insert_resource(ActionState::default());

    app.add_systems(OnEnter(AppState::Game), action_state_setup);
    app.add_systems(OnExit(AppState::Game), action_state_cleanup);

    app.observe(summon_action_execute); //TODO only attach to a target, on  self agent ?
    app.observe(summon_action_finish);
}

//MAYBE change how resource is inserted ? this will cause an issue if the player is on multiple boards, i don't think it will happen
//MAYBE Also maybe just insert it when the player join a board, but honestly it's negligable memory save
fn action_state_setup(mut commands: Commands) {
    commands.insert_resource(ActionState::default());
}

fn action_state_cleanup(mut commands: Commands) {
    commands.remove_resource::<ActionState>();
}
