// Create verify_token.rs in your network-service directory
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub role: String, 
    pub exp: u64,
}

const JWT_SECRET: &str = "secret123";

fn main() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJhZG1pbiIsImV4cCI6MTc2Mjk4MDM3NH0.d-Xv6C_qK6R2GPpI9tmwW9IjoquZYbb3DkA5w3Ad2cs";
    
    println!("Testing token verification...");
    println!("Token: {}", token);
    println!("JWT Secret: {}", JWT_SECRET);
    
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    
    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            println!("✅ SUCCESS: Token is valid!");
            println!("User: {}", token_data.claims.sub);
            println!("Role: {}", token_data.claims.role);
            println!("Exp: {}", token_data.claims.exp);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            println!("This means the JWT_SECRET doesn't match!");
        }
    }
}