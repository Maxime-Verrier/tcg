use bevy::ecs::system::SystemId;
pub use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ActionState {
    current: Vec<Action>,
}

#[derive(Clone)]
pub struct Action {
    actionner: Entity,
    entities: Vec<Entity>,
    on_execute: SystemId<ActionInput>,
    on_canceled: Option<SystemId<ActionInput>>,
    on_finish: SystemId<ActionInput>,
}

impl Action {
    pub fn new(
        actionner: Entity,
        entities: Vec<Entity>,
        on_execute: SystemId<ActionInput>,
        on_finish: SystemId<ActionInput>,
        on_canceled: Option<SystemId<ActionInput>>,
    ) -> Self {
        Self {
            actionner,
            entities,
            on_execute,
            on_canceled,
            on_finish,
        }
    }
}

pub struct ActionInput {
    pub actionner: Entity,
    pub entities: Vec<Entity>,
}

impl ActionInput {
    pub fn new(actionner: Entity, entities: Vec<Entity>) -> Self {
        Self {
            actionner,
            entities,
        }
    }
}

#[derive(Event)]
pub struct ActionExecuteEvent(pub Action);

#[derive(Event)]
pub struct ActionFinishEvent;

pub(crate) fn action_execute_observer(
    trigger: Trigger<ActionExecuteEvent>,
    mut commands: Commands,
    mut query: Query<&mut ActionState>,
) {
    if let Ok(mut action_state) = query.get_mut(trigger.event().0.actionner) {
        let action = trigger.event().0.clone();

        action_state.current.push(action);
        if action_state.current.len() == 1 {
            let action = action_state.current.first().unwrap();

            commands.run_system_with_input(
                action.on_execute,
                ActionInput::new(action.actionner, action.entities.clone()),
            );
        }
    }
}

pub(crate) fn action_finish_observer(
    trigger: Trigger<ActionFinishEvent>,
    mut commands: Commands,
    mut query: Query<&mut ActionState>,
) {
    if let Ok(mut action_state) = query.get_mut(trigger.entity()) {
        if let Some(action) = action_state.current.pop() {
            commands.run_system_with_input(
                action.on_finish,
                ActionInput::new(action.actionner, action.entities),
            );
        }
    }
}
