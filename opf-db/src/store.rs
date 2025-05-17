use std::collections::HashMap;

use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

use opf_models::event::Event;
use opf_models::{Group, Link, Target, Suggestion};

#[derive(Debug)]
pub struct DB {
    pub db_tx: UnboundedSender<Event>,
    pub db_id: i32,
    pub targets: RwLock<HashMap<i32, Target>>,
    pub groups: RwLock<HashMap<i32, Group>>,
    pub links: RwLock<HashMap<i32, Link>>,
    pub suggestions: RwLock<HashMap<i32, Suggestion>>,
}

impl DB {
    pub fn new(db_tx: UnboundedSender<Event>) -> Self {
        Self {
            db_tx,
            db_id: 0,
            targets: RwLock::new(HashMap::new()),
            groups: RwLock::new(HashMap::new()),
            links: RwLock::new(HashMap::new()),
            suggestions: RwLock::new(HashMap::new()),
        }
    }
}
