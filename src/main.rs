mod connector;
mod boundary;
mod codec_data;
mod generated;
mod simulation;

use std::time::Duration;
use connector::connection_handler::{ConnectionHandler};
use bevy::{prelude::*, log::LogPlugin, app::ScheduleRunnerSettings};
use simulation::{resources::{dimensions::Dimensions, message_router::MessageRouter, players::Players}, plugins::message_handlers::MessageHandlersPlugin};

fn main() {
    let dimensions = Dimensions::new();
    let players = Players::new();

    let (main_boundary, _connection_handler) = ConnectionHandler::bootstrap();

    let message_router = MessageRouter::new(main_boundary);

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(50)))

        .insert_resource(dimensions)
        .insert_resource(players)
        .insert_resource(message_router)

        //Bevy Plugins
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())

        //Custom Plugins
        .add_plugin(MessageHandlersPlugin)

        .run();
}

