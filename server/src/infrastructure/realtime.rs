use crate::domain::value_objects::realtime::RealtimeEvent;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct RealtimeHub {
    pub tx: broadcast::Sender<RealtimeEvent>,
}

impl RealtimeHub {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self { tx }
    }

    pub fn broadcast(&self, event: RealtimeEvent) {
        let _ = self.tx.send(event);
    }
}

pub type SharedRealtimeHub = Arc<RealtimeHub>;
