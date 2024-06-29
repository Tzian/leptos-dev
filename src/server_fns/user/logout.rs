use super::*;

#[server(UserLogout, "/logout")]
pub async fn logout_user() -> Result<(), ServerFnError<ServicesError>>
{
	use actix_web::{cookie::{time::OffsetDateTime, Cookie},
	                http::{header, header::HeaderValue}};
	use leptos_actix::ResponseOptions;
	use time::Duration;

	let response = expect_context::<ResponseOptions>();
	let cookie =
		Cookie::build("leptos_access_token", "".to_string()).path("/")
		                                                    .http_only(true)
		                                                    .expires(OffsetDateTime::now_utc() - Duration::days(1))
		                                                    .finish();

	if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
	{
		response.insert_header(header::SET_COOKIE, cookie);
	}

	leptos_actix::redirect("/login");

	Ok(())
}
