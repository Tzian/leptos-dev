use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims
{
	pub exp:   usize,
	pub iat:   usize,
	pub email: String,
	pub id:    i32
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error>
{
	println!("encoding jwt");
	let now = Utc::now();
	let expiry = Duration::minutes(60);

	let claims = Claims { exp: (now + expiry).timestamp() as usize,
	                      iat: now.timestamp() as usize,
	                      id,
	                      email };

	dotenvy::dotenv().ok();

	let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not found, please set it in .env file");

	encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error>
{
	dotenvy::dotenv().ok();

	let secret = std::env::var("JWT_SECRET").map_err(|_| -> jsonwebtoken::errors::Error {
		                                        jsonwebtoken::errors::ErrorKind::InvalidToken.into()
	                                        })?;
	let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> =
		decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

	claim_data
}
