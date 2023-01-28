use std::collections::HashMap;
use super::{dimension::Dimension};

pub struct ServerState {
    pub dimensions: HashMap<String, Dimension>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            dimensions: HashMap::from([("minecraft:overworld".into(), Dimension::default())]),
        }
    }
}

