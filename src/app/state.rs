use actix_web::FromRequest;
use futures_util::future::{ok, Ready};
use ls_service::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState
{
	pub conn: DatabaseConnection
}

impl FromRequest for AppState
{
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future
	{
		let data = req.app_data::<actix_web::web::Data<AppState>>().expect("Could not get data from request");

		ok(data.get_ref().clone())
	}
}