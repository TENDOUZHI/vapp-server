use chrono::{Duration, Local};
use jsonwebtoken::{encode, errors::Error, Algorithm, EncodingKey, Header, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
    aud: String,         // Optional. Audience
    exp: i64,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64,          // Optional. Issued at (as UTC timestamp)
    nbf: i64,          // Optional. Not Before (as UTC timestamp)
}

pub fn genarate_token(username: String) -> Result<String, Error> {
    let now = Local::now().timestamp_millis();
    let exp_time = now + Duration::days(1).num_milliseconds();
    let payload = Claims {
        iss: "Vapp_Actix".to_owned(),
        sub: "Normal users".to_owned(),
        aud: username,
        exp: exp_time,
        iat: now,
        nbf: now,
    };
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret("secret".as_ref()),
    );
    token
}

pub fn check_token(token: String) -> bool {
    let now = Local::now().timestamp_millis();
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match token_message {
        Ok(data) => {
            if now > data.claims.exp {
                return false;
            } else {
                return true;
            }
        },
        Err(_) => false,
    }
}
