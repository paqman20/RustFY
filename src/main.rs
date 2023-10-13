use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::{string, vec};
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


// Json Structures
#[derive(Serialize, Deserialize, Debug)]
struct Tokens {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

// Main Function
#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Construct the URL for the authentication
    let client_id = "f5a249051b84495f80f9988b9d1c8b90";
    let client_secret = "4a76f53959d14f1b84a779eb4b44de91";
//    let authentication_struct = fetch_authentication(client_id, client_secret).await?;
//    println!("{:?}", authentication_struct.access_token);
    let random_string = generate_random_string();
    let code_challeneger = generate_code_verifier();
    println!("{}", code_challeneger);
    Ok(())
}

async fn fetch_authentication_client_credentials(client_id: &str, client_secret: &str,) -> Result<Tokens, Box<dyn StdError>> {
    let body = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}",
        client_id, client_secret,
    );

    let client = reqwest::Client::new();

    let authentication_token = client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?
        .json::<Tokens>()
        .await?;

    Ok(authentication_token)
}

fn generate_code_verifier() -> String {
    let sha256 = Sha256::new;
    let code_verifer = generate_code_verifier();
    let code_challenger = sha256().finalize();
    
    return format!("{:x}", code_challenger); 
}

fn generate_random_string() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
 return rand_string
}