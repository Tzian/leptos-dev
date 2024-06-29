use db_entities::{user, user::Entity as User};
use sha256::digest;

use super::*;
use crate::utils::jwt::encode_jwt;

pub struct UserQuery;

impl UserQuery
{
	// Find a user by id
	pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<user::Model, ServicesError>
	{
		User::find_by_id(id).one(db)
		                    .await
		                    .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		                    .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })
	}

	// Find user by email
	pub async fn find_user_by_email(db: &DbConn, email: &str) -> Result<user::Model, ServicesError>
	{
		User::find().filter(user::Column::Email.eq(email))
		            .one(db)
		            .await
		            .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		            .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })
	}

	// Find user by username
	pub async fn find_user_by_username(db: &DbConn, username: &str) -> Result<user::Model, ServicesError>
	{
		User::find().filter(user::Column::Username.eq(username))
		            .one(db)
		            .await
		            .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		            .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })
	}

	// Find all users
	pub async fn find_all_users(db: &DbConn) -> Result<Vec<user::Model>, ServicesError>
	{
		User::find().all(db).await.map_err(|e| ServicesError::DbError { db_err: e.to_string() })
	}

	/// Find a user by username or email and verifies password match returning an encoded JWT token.
	pub async fn authenticate_user(db: &DbConn, identity: &str, password: &str) -> Result<String, ServicesError>
	{
		let password_hash = digest(password);

		let user = User::find().filter(Condition::all().add(user::Column::PasswordHash.eq(password_hash))
		                                               .add(Condition::any().add(user::Column::Username.eq(identity))
		                                                                    .add(user::Column::Email.eq(identity))))
		                       .one(db)
		                       .await
		                       .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		                       .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })?;

		let encoded = encode_jwt(user.email.clone(), user.id).map_err(|e| {
			                                                     ServicesError::ConversionError { conv_err:
				                                                                                      e.to_string() }
		                                                     })?;
		Ok(encoded)
	}
}
