mod action;

pub use action::*;

pub struct ActionPlugin;

impl bevy::app::Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionExecuteEvent>();
        app.add_event::<ActionFinishEvent>();
        app.observe(action_execute_observer);
        app.observe(action_finish_observer);
    }
}
