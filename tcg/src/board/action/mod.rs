mod action;
mod summon;

pub use action::*;
pub use summon::*;

pub(crate) fn board_action_plugin(app: &mut bevy::app::App) {
    app.observe(summon_action_execute); //TODO only attach to a target, on  self agent ?
    app.observe(summon_action_finish);
}
