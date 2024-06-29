use super::*;

#[server(UserLogin, "/login")]
pub async fn login(identity: String, password: String) -> Result<(), ServerFnError<ServicesError>>
{
	use actix_web::{cookie::{time::Duration, Cookie},
	                http::{header, header::HeaderValue}};
	use db_services::UserQuery;
	use leptos_actix::ResponseOptions;

	use crate::app::state::AppState;

	let state: AppState =
		leptos_actix::extract().await.map_err(|e| {
			                              ServerFnError::WrappedServerError(ServicesError::ConnectionError{con_err: e.to_string()})
		                              })?;
	let conn = state.conn;

	let reply = UserQuery::authenticate_user(&conn, &identity, &password).await
	.map_err(|e| ServerFnError::WrappedServerError(ServicesError::QueryError{q_err: e.to_string()}))?;

	let response = expect_context::<ResponseOptions>();

	let cookie =
		Cookie::build("leptos_access_token", reply).path("/").http_only(true).max_age(Duration::minutes(60)).finish();

	if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
	{
		response.insert_header(header::SET_COOKIE, cookie);
	}

	leptos_actix::redirect("/dashboard");

	Ok(())
}
