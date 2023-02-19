use std::time::Instant;

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct SentKeepAlive {
    pub id: i64,
    pub time: Instant,
}

#[derive(Debug, Component)]
pub struct Ping(pub i32);