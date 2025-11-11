pub struct GetNetworkByIdQuery {
    pub id: i32,
}

impl GetNetworkByIdQuery {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub struct GetAllNetworksQuery;

impl GetAllNetworksQuery {
    pub fn new() -> Self {
        Self
    }
}
