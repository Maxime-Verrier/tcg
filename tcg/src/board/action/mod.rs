mod action;
mod summon;

pub use action::*;
use bevy_replicon::client::ClientSet;
pub use summon::*;

pub(crate) fn board_action_plugin(app: &mut bevy::app::App) {
    app.observe(summon_action_execute); //TODO only attach to a target, on  self agent ?
    app.observe(summon_action_finish);
    app.add_systems(
        Update,
        action_state_agent_observer.after(ClientSet::Receive),
    );
}
