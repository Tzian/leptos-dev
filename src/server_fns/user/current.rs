use super::*;

#[server]
pub async fn get_current_user(token: Option<String>) -> Result<Option<DisplayUserModel>, ServerFnError<ServicesError>>
{
	use db_entities::user::Model;
	use db_services::{utils::jwt::decode_jwt, UserQuery};

	use crate::{app::state::AppState, server_fns::user::logout::logout_user};

	let Some(token_value) = token
	else
	{
		return Ok(None);
	};

	// decode token and get user id and email from it
	let Ok(data_value) = decode_jwt(token_value)
	else
	{
		let _ = logout_user().await;
		return Ok(None);
	};

	let state: AppState =
		leptos_actix::extract().await.map_err(|e| {
			                              ServerFnError::WrappedServerError(ServicesError::ConnectionError{con_err: e.to_string()})
		                              })?;
	let conn = state.conn;

	// find user by id
	let user: Model =
		UserQuery::find_user_by_id(&conn, data_value.claims.id).await.map_err(|e| {
			                                                              ServerFnError::WrappedServerError(ServicesError::QueryError{q_err: e.to_string()})
		                                                              })?;

	// check if user id and email match the token
	if user.id != data_value.claims.id || user.email != data_value.claims.email
	{
		return Ok(None);
	}

	let current = DisplayUserModel { id: user.id, username: user.username, email: user.email };

	Ok(Some(current))
}
