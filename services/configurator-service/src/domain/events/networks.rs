use crate::domain::entities::networks::Network;

#[derive(Debug, Clone)]
pub struct NetworkCreatedEvent(pub Network);

#[derive(Debug, Clone)]
pub struct NetworkUpdatedEvent(pub Network);

#[derive(Debug, Clone)]
pub struct NetworkDeletedEvent(pub i32);

pub fn create_network_created_event(network: Network) -> NetworkCreatedEvent {
    NetworkCreatedEvent(network)
}

pub fn create_network_updated_event(network: Network) -> NetworkUpdatedEvent {
    NetworkUpdatedEvent(network)
}

pub fn create_network_deleted_event(id: i32) -> NetworkDeletedEvent {
    NetworkDeletedEvent(id)
}
