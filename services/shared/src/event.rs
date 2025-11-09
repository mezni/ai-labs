use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event<T> {
    pub id: String,
    pub timestamp: i64,
    pub event_type: String,
    pub payload: T,
}

impl<T> Event<T> {
    pub fn new(event_type: &str, payload: T) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            event_type: event_type.to_string(),
            payload,
        }
    }
}
