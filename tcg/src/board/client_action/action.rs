#[cfg(all(feature = "render", feature = "client"))]
pub use cfg_client_action::*;

#[cfg(all(feature = "render", feature = "client"))]
mod cfg_client_action {
    use bevy::prelude::*;

    /// A client action is a multipart action that can be executed and canceled
    /// They are not commiting and do not impact the game state until fully executed
    /// Mostly used for UI/FX interactins leading to a game action
    /// Exemple: selecting a card to then play it on x slot, require multiple steps and do nto impact the game state until fully commited
    #[derive(Event)]
    pub struct ClientAction {
        action_event: Box<dyn ClientActionFunc + 'static + Sync + Send>,
        cancel_event: Box<dyn ClientActionFunc + 'static + Sync + Send>,
    }

    impl ClientAction {
        pub fn new(
            action_event: Box<dyn ClientActionFunc + 'static + Sync + Send>,
            cancel_event: Box<dyn ClientActionFunc + 'static + Sync + Send>,
        ) -> Self {
            Self {
                action_event,
                cancel_event,
            }
        }
    }

    /// Allow to store event by erasing the type of the event
    /// The event will be cloned when triggered
    pub trait ClientActionFunc {
        fn send_deferred(&self, cmd: &mut Commands);
    }

    impl<T: Event + Clone> ClientActionFunc for T {
        fn send_deferred(&self, cmd: &mut Commands) {
            let cloned = self.clone();

            cmd.add(move |w: &mut World| {
                w.trigger(cloned);
            });
        }
    }

    /// State to store the current client action
    /// For ergonomic reason, only one action can be executed at a time which will allow to clean fx/ui/interaction of non commited actions
    /// Exemple: trying to summon a card, not commiting then selecting annother card will result the summon action to be canceld, which will clean the fx/ui related to the summon action
    #[derive(Resource, Default)]
    pub struct ClientActionState {
        pub current: Option<ClientAction>,
    }

    impl ClientActionState {
        pub(crate) fn execute_action(&mut self, commands: &mut Commands, action: ClientAction) {
            if let Some(action) = &self.current {
                action.cancel_event.send_deferred(commands);
            }
            action.action_event.send_deferred(commands);
            self.current = Some(action);
        }
    }
}
