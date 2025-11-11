use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto};

pub struct CreateNetworkCommand {
    pub dto: CreateNetworkDto,
}

impl CreateNetworkCommand {
    pub fn new(dto: CreateNetworkDto) -> Self {
        Self { dto }
    }
}

pub struct UpdateNetworkCommand {
    pub dto: UpdateNetworkDto,
}

impl UpdateNetworkCommand {
    pub fn new(dto: UpdateNetworkDto) -> Self {
        Self { dto }
    }
}
