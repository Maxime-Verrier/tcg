use bevy::{ecs::entity::MapEntities, prelude::*};
use bevy_replicon::{
    core::ClientId,
    prelude::{SendMode, ToClients},
};
use epithet::units::{RenderRegistry, UnitRegistry};
use serde::{Deserialize, Serialize};

use super::{Card, CardAttribute};

#[derive(Component)]
pub struct CardVisibility {
    // A vec as insertion are really rare and can be push to back and removal are almost non existent where as iteration is common at any mutation
    pub visible_to: Vec<ClientId>,

    // We want to keep the list of clients that can see this card while in situation where everyone can see the card so it's easy to keep track of which clients see the cards when back to a state where the visibility is private again
    pub visible_to_all: bool,
}

impl CardVisibility {
    pub fn new(visible_to: Vec<ClientId>, visible_to_all: bool) -> Self {
        Self {
            visible_to,
            visible_to_all,
        }
    }
}

//TODO change it to generic when replicon support component visbility per entity/clients
#[derive(Event, Serialize, Deserialize)]
pub struct CardAttributePacket {
    pub card: Entity,
    pub attribute: CardAttribute,
    pub remove: bool,
}

impl MapEntities for CardAttributePacket {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.card = entity_mapper.map_entity(self.card);
    }
}

//TODO make it OnMutate observer when bevy supports it
pub(crate) fn card_visibility_observer(
    mut event_writter: EventWriter<ToClients<CardAttributePacket>>,
    query: Query<
        (Entity, &CardVisibility, &CardAttribute),
        Or<(Added<CardAttribute>, Added<CardVisibility>)>,
    >,
) {
    for (entity, visibility, attribute) in query.iter() {
        if visibility.visible_to_all {
            event_writter.send(ToClients {
                mode: SendMode::Broadcast,
                event: CardAttributePacket {
                    card: entity,
                    attribute: attribute.clone(),
                    remove: false,
                },
            });
        } else {
            for client_id in visibility.visible_to.iter() {
                event_writter.send(ToClients {
                    mode: SendMode::Direct(*client_id),
                    event: CardAttributePacket {
                        card: entity,
                        attribute: attribute.clone(),
                        remove: false,
                    },
                });
            }
        }
    }
}

pub(crate) fn on_card_visibility_event(
    mut commands: Commands,
    mut reader: EventReader<CardAttributePacket>,
    renders: Res<RenderRegistry>,
    units: Res<UnitRegistry>,
) {
    for packet in reader.read() {
        if packet.remove {
            commands.entity(packet.card).remove::<CardAttribute>();
        } else {
            let mut entity_commands: bevy::ecs::system::EntityCommands<'_> =
                commands.entity(packet.card);

            entity_commands.insert(packet.attribute.clone());

            #[cfg(feature = "render")]
            {
                entity_commands.despawn_descendants();

                //TODO just replace that with an event/observer/oneshot that recreate it somewhere else
                renders.create_render(units.get_id::<Card>(), &mut commands, packet.card);
            }
        }
    }
}
