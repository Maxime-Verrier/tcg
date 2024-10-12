pub use bevy::prelude::*;

/// Allow to store event by erasing the type of the event
/// It's need to be clonable and will get cloned when the event is sent
pub trait ActionEvent {
    fn send_deferred(&self, cmd: &mut Commands);
}

impl<T: Event + Clone> ActionEvent for T {
    fn send_deferred(&self, cmd: &mut Commands) {
        let cloned = self.clone();

        cmd.add(move |w: &mut World| {
            w.trigger(cloned);
        });
    }
}
#[derive(Event)]
pub struct Action {
    action_event: Box<dyn ActionEvent + 'static + Sync + Send>,
    cancel_event: Box<dyn ActionEvent + 'static + Sync + Send>,
}

impl Action {
    pub fn new(
        action_event: Box<dyn ActionEvent + 'static + Sync + Send>,
        cancel_event: Box<dyn ActionEvent + 'static + Sync + Send>,
    ) -> Self {
        Self {
            action_event,
            cancel_event,
        }
    }
}

#[derive(Resource, Default)]
pub struct ActionState {
    pub current: Option<Action>,
}

impl ActionState {
    pub(crate) fn execute_action(&mut self, commands: &mut Commands, action: Action) {
        if let Some(action) = &self.current {
            action.cancel_event.send_deferred(commands);
        }
        action.action_event.send_deferred(commands);
        self.current = Some(action);
    }
}
