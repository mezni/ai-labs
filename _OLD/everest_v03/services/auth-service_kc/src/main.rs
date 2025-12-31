mod keycloak_client;

use keycloak_client::{HttpKeycloakClient, KeycloakClient};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check your environment variables or hardcoded strings carefully
    let client = HttpKeycloakClient::new(
        "http://localhost:5080".into(),
        "myrealm".into(),
        "backend-admin".into(),        
        "backend-admin-secret".into(), 
        "auth-client".into(),          
    );

    println!("Step 1: Admin creating user...");
    
    // We use a unique username to avoid "User already exists" (409 Conflict)
    let username = format!("rust_user_{}", chrono::Utc::now().timestamp());

    match client.create_user(
        "test@example.com", 
        &username, 
        "Password123!", 
        None
    ).await {
        Ok(id) => {
            println!("✅ Success! User Created with ID: {}", id);
            
            // Testing Role Assignment
            println!("Step 2: Assigning 'user' role...");
            match client.assign_role(&id, "user").await {
                Ok(_) => println!("✅ Role assigned!"),
                Err(e) => println!("❌ Role assignment failed (ensure role 'user' exists): {}", e),
            }
        },
        Err(e) => {
            println!("❌ Create User Failed!");
            println!("Detail: {}", e);
            println!("\nPossible Fix: Go to Keycloak -> Clients -> backend-admin -> Service Account Roles -> Assign 'manage-users' from 'realm-management' client.");
        }
    }

    Ok(())
}