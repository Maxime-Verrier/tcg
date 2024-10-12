use bevy::prelude::*;
use bevy_replicon::prelude::{RepliconChannels, ToClients};
use epithet::{
    net::{client_setup, server_listener_setup, AuthEvent, AuthManager, NetState},
    utils::{GameEntity, LevelEntity},
};

use crate::state::AppState;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

//TODO idc anymore just let me do it for 1 commit so can finish the real feature of this commit instead of needing to do another feature that willt takes hours
#[derive(Component)]
pub struct MagicNumber(u8);

pub fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), GameEntity));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            LevelEntity,
        ))
        .with_children(|parent| {
            spawn_button_with_text(parent, &asset_server, "Single Player", 0);
            spawn_button_with_text(parent, &asset_server, "Server", 1);
            spawn_button_with_text(parent, &asset_server, "Join Server", 2);
            spawn_button_with_text(parent, &asset_server, "Quit", 3);
        });
}

// Helper function to spawn a button with text for the main menu
#[inline]
fn spawn_button_with_text(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    magic_number: u8,
) {
    parent
        .spawn((
            ButtonBundle {
                background_color: NORMAL_BUTTON.into(),
                ..Default::default()
            },
            MagicNumber(magic_number),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
        });
}

pub(crate) fn main_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MagicNumber),
        (Changed<Interaction>, With<Button>),
    >,
    mut states: ResMut<NextState<AppState>>,
    mut net_states: ResMut<NextState<NetState>>,
    mut commands: Commands,
    channels: Res<RepliconChannels>,
    auth_manager: ResMut<AuthManager>,
    mut writer: EventWriter<ToClients<AuthEvent>>,
) {
    let channels = channels.into_inner();
    let auth_manager = auth_manager.into_inner();

    for (interaction, mut color, magic_number) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                if magic_number.0 == 0 {
                    net_states.set(NetState::ListeningServer);
                    states.set(AppState::Game);
                    server_listener_setup(&mut commands, channels, auth_manager, &mut writer);
                //TODO use result
                } else if magic_number.0 == 1 {
                    net_states.set(NetState::ListeningServer);
                    states.set(AppState::Game);
                    server_listener_setup(&mut commands, channels, auth_manager, &mut writer);
                //TODO use result
                } else if magic_number.0 == 2 {
                    net_states.set(NetState::Client);
                    states.set(AppState::Game);
                    client_setup(&mut commands, channels); //TODO use result
                } else if magic_number.0 == 3 {
                    std::process::exit(0);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
