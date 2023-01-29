mod connector;
mod boundary;
mod data;
mod simulation;
mod utils;

use std::time::Duration;

use boundary::{main_boundary::MainBoundary, message::OutboundMessage};
use connector::connection_handler::{ConnectionHandler};
use bevy::{prelude::*, log::LogPlugin, app::ScheduleRunnerSettings};
use data::packets::play::{OutboundPlay, KeepAlive};
use simulation::{dimensions::Dimensions, player::PlayerInfo};
use utils::send_init_messages::send_init_messages;

fn message_handler(mut commands: Commands, mut main_boundary: ResMut<MainBoundary>, dimensions: Res<Dimensions>) {
    for ident_message in main_boundary.recieve_all_messages() {
        match ident_message.message {
            boundary::message::InboundMessage::InitConnection => {
                info!("Player Connected: {:?}", ident_message.uuid);

                commands.spawn((PlayerInfo { uuid: ident_message.uuid },));

                send_init_messages(ident_message.uuid, &mut main_boundary, &dimensions);
            },
            boundary::message::InboundMessage::Play(packet) => {
                info!("{:?}", packet);
            },
            boundary::message::InboundMessage::TermConnection => {
                info!("Player Disconnected: {:?}", ident_message.uuid);
            },
        }
    }
}

fn keep_alive(main_boundary: ResMut<MainBoundary>, query: Query<(&PlayerInfo,)>) {
    for (player_info,) in query.iter() {
        main_boundary.send_message(player_info.uuid, OutboundMessage::Play(OutboundPlay::KeepAlive(KeepAlive {
            keep_alive_id: 1,
        })));
    }
}

fn main() {
    let dimensions = Dimensions::new();

    let (main_boundary, _connection_handler) = ConnectionHandler::bootstrap();

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_micros(625)))

        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())

        .insert_resource(dimensions)
        .insert_resource(main_boundary)

        .add_system(message_handler)
        .add_system(keep_alive)

        .run();
}

