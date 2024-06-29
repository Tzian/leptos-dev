use db_entities::{post, post::Entity as Post};

use super::*;

pub struct PostQuery;

impl PostQuery
{
	pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<post::Model, ServicesError>
	{
		Post::find_by_id(id).one(db)
		                    .await
		                    .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		                    .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })
	}
}
