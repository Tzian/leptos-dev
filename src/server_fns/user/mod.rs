pub mod current;
pub mod login;
pub mod logout;
pub mod register;

use std::str::FromStr;

use db_utils::errors::ServicesError;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DisplayUserModel
{
	pub id:       i32,
	pub username: String,
	pub email:    String
}

impl Default for DisplayUserModel
{
	fn default() -> Self
	{
		Self { id: -1, username: "".to_string(), email: "".to_string() }
	}
}

impl FromStr for DisplayUserModel
{
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		let usr: DisplayUserModel = serde_json::from_str(s).map_err(|e| e.to_string())?;
		Ok(usr)
	}
}

impl std::fmt::Display for DisplayUserModel
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{}", serde_json::to_string(self).expect("Could not serialize DisplayUserModel"))
	}
}
