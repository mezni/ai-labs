use crate::domain::entities::network::Network;
use crate::domain::value_objects::{Email, PhoneNumber, Address, NetworkType};

/// Domain service for network-related business rules.
/// This service contains *pure business logic* — no database, no I/O.
pub struct NetworkDomainService;

impl NetworkDomainService {
    /// Validate whether a network can be created.
    /// This enforces business invariants at the domain level.
    pub fn validate_creation(network: &Network) -> Result<(), String> {
        // Example rule: Name must not be empty
        if network.name.trim().is_empty() {
            return Err("Network name cannot be empty.".to_string());
        }

        // Example rule: Network type must be valid (Individual or Company)
        match &network.network_type {
            NetworkType::Individual | NetworkType::Company => {}
        }

        // Validate email if present
        if let Some(email) = &network.contact_email {
            Email::new(email.value().to_string())
                .map_err(|_| "Invalid contact email address.".to_string())?;
        }

        // Validate phone if present
        if let Some(phone) = &network.phone_number {
            PhoneNumber::new(phone.value().to_string())
                .map_err(|_| "Invalid phone number format.".to_string())?;
        }

        // Validate address if present
        if let Some(address) = &network.address {
            Address::new(address.value().to_string())
                .map_err(|_| "Invalid address provided.".to_string())?;
        }

        Ok(())
    }

    /// Example business rule: Check if two networks conflict.
    /// (e.g., same name and type should not coexist)
    pub fn conflicts_with(existing: &Network, new: &Network) -> bool {
        existing.name.eq_ignore_ascii_case(&new.name)
            && existing.network_type == new.network_type
    }
}
