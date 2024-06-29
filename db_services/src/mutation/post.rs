use db_entities::{post, post::Entity as Post};
use sea_orm::*;

use super::*;

pub struct PostMutation;

impl PostMutation
{
	pub async fn create_post(db: &DbConn, form_data: post::Model) -> Result<post::ActiveModel, ServicesError>
	{
		let new_post = post::ActiveModel { user_id: Set(form_data.user_id.to_owned()),
		                                   title: Set(form_data.title.to_owned()),
		                                   text: Set(form_data.text.to_owned()),
		                                   ..Default::default() };

		new_post.save(db).await.map_err(|e| ServicesError::DbError { db_err: e.to_string() })
	}

	pub async fn update_post_by_id(db: &DbConn, id: i32, form_data: post::Model) -> Result<post::Model, ServicesError>
	{
		let post = Post::find_by_id(id).one(db)
		                               .await
		                               .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		                               .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })?;

		let post: post::ActiveModel = post.into();

		let updated_post = post::ActiveModel { id:      post.id,
		                                       user_id: Set(form_data.user_id.to_owned()),
		                                       title:   Set(form_data.title.to_owned()),
		                                       text:    Set(form_data.text.to_owned()) };

		updated_post.update(db).await.map_err(|e| ServicesError::DbError { db_err: e.to_string() })
	}

	pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, ServicesError>
	{
		let post = Post::find_by_id(id).one(db)
		                               .await
		                               .map_err(|e| ServicesError::DbError { db_err: e.to_string() })?
		                               .ok_or(ServicesError::QueryError { q_err: "No Record Found".to_string() })?;

		let post: post::ActiveModel = post.into();
		post.delete(db).await.map_err(|e| ServicesError::DbError { db_err: e.to_string() })
	}

	pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, ServicesError>
	{
		Post::delete_many().exec(db).await.map_err(|e| ServicesError::DbError { db_err: e.to_string() })
	}
}
