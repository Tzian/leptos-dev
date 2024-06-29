#[cfg(feature = "ssr")]
use chrono::NaiveDate;

use super::*;

#[server(RegisterUser, "/register")]
pub async fn register_user(first_name: String,
                           last_name: String,
                           username: String,
                           email: String,
                           date_of_birth: String,
                           password: String)
                           -> Result<i32, ServerFnError<String>>
{
	use actix_web::{cookie::Cookie,
	                http::{header, header::HeaderValue}};
	use db_entities::user::RegisterUserModel;
	use db_services::{UserMutation, UserQuery};
	use leptos_actix::ResponseOptions;
	use time::Duration;

	use crate::app::state::AppState;

	println!("register user");

	// check inputs are valid
	process_inputs(
	               first_name.clone(),
	               last_name.clone(),
	               username.clone(),
	               date_of_birth.clone(),
	               email.clone(),
	               password.clone()
	).await?;

	let state: AppState = leptos_actix::extract().await.map_err(|e| e.to_string())?;
	let conn = state.conn;

	let dob = NaiveDate::parse_from_str(&date_of_birth.clone(), "%Y-%m-%d").map_err(|e| e.to_string())?;

	let form = RegisterUserModel { username: username.clone(),
	                               first_name,
	                               last_name,
	                               email: email.clone(),
	                               date_of_birth: dob,
	                               password: password.clone() };

	let user_id: i32 = UserMutation::create_new_user(&conn, form).await
	                                                             .map_err(|e| {
		                                                             if e.to_string().contains("users_username_key")
		                                                             {
			                                                             ServerFnError::WrappedServerError(
				                                                                                   "Error:- Username \
				                                                                                    already in use"
				                                                                                       .to_string()
				)
		                                                             }
		                                                             else if e.to_string().contains("users_email_key")
		                                                             {
			                                                             ServerFnError::WrappedServerError(
			                                                                                          "Error:- Email \
			                                                                                           already use"
			                                                                                              .to_string()
			)
		                                                             }
		                                                             else
		                                                             {
			                                                             ServerFnError::WrappedServerError(e.to_string())
		                                                             }
	                                                             })?
	                                                             .id;

	let reply = UserQuery::authenticate_user(&conn, &username, &password).await.map_err(|e| e.to_string())?;

	let response = expect_context::<ResponseOptions>();
	let cookie =
		Cookie::build("leptos_access_token", reply).path("/").http_only(true).max_age(Duration::minutes(10)).finish();

	if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
	{
		response.insert_header(header::SET_COOKIE, cookie);
	}

	leptos_actix::redirect("/dashboard");

	Ok(user_id)
}

#[server]
async fn process_inputs(first_name: String,
                        last_name: String,
                        username: String,
                        date_of_birth: String,
                        email: String,
                        password: String)
                        -> Result<(), ServerFnError<String>>
{
	use chrono::{Datelike, Utc};

	let day: u32 = date_of_birth.split("-").collect::<Vec<&str>>()[2].parse::<u32>().map_err(|e| e.to_string())?;
	let month: u32 = date_of_birth.split("-").collect::<Vec<&str>>()[1].parse::<u32>().map_err(|e| e.to_string())?;
	let year: i32 = date_of_birth.split("-").collect::<Vec<&str>>()[0].parse::<i32>().map_err(|e| e.to_string())?;

	let now = Utc::now();
	let current_year = now.year();
	let current_month = now.month();
	let current_day = now.day();

	let age = current_year - year;

	match age.cmp(&18)
	{
		std::cmp::Ordering::Less =>
		{
			return Err(ServerFnError::WrappedServerError("Error:- You must be at least 18 years old to register".to_string()));
		}
		std::cmp::Ordering::Equal =>
		{
			if month > current_month || (month == current_month && day > current_day)
			{
				return Err(ServerFnError::WrappedServerError("Error:- You must be at least 18 years old to register".to_string()));
			}
		}
		std::cmp::Ordering::Greater =>
		{}
	}

	// check password meets requirements
	if password.len() < 8
	{
		println!("password too short");
		return Err(ServerFnError::WrappedServerError("Error:- Password must be at least 8 characters long".to_string()));
	}
	else if !password.contains(char::is_lowercase)
	{
		println!("password no lowercase");
		return Err(ServerFnError::WrappedServerError("Error:- Password must contain at least one lowercase letter".to_string()));
	}
	else if !password.contains(char::is_uppercase)
	{
		return Err(ServerFnError::WrappedServerError("Error:- Password must contain at least one uppercase letter".to_string()));
	}
	else if !password.contains(char::is_numeric)
	{
		return Err(ServerFnError::WrappedServerError("Error:- Password must contain at least one number".to_string()));
	}
	else if !password.contains(|c: char| "(!@#£$%^&*)".contains(c))
	{
		return Err(ServerFnError::WrappedServerError(
		                                             "Error:- Password must contain at least one of these symbols \
		                                              (!@#£$%^&*)"
		                                                    .to_string()
		));
	}

	// check username is valid length
	if username.len() < 6 || username.len() > 30
	{
		return Err(ServerFnError::WrappedServerError("Error:- Usernames must be between 6 and 30 characters long".to_string()));
	}

	// check first name is at least 2 characters long
	if first_name.len() < 2 || last_name.len() < 2
	{
		return Err(ServerFnError::WrappedServerError(
		                                             "Error:- First and last names must be at least 2 characters \
		                                              long"
		                                             .to_string()
		));
	}

	// TODO: setup sending email to verify email address

	// check email is valid
	if !email.contains("@") || !email.contains(".")
	{
		return Err(ServerFnError::WrappedServerError("Error:- Invalid email address".to_string()));
	}

	Ok(())
}
