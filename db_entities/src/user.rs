use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, FromJsonQueryResult)]
#[sea_orm(table_name = "users")]
pub struct Model
{
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id:            i32,
	#[sea_orm(unique)]
	pub username:      String,
	pub first_name:    String,
	pub last_name:     String,
	#[sea_orm(unique)]
	pub email:         String,
	pub password_hash: String,
	pub date_of_birth: NaiveDate,
	#[sea_orm(unique)]
	pub uuid:          Uuid,
	pub created_at:    NaiveDateTime,
	pub updated_at:    NaiveDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
	#[sea_orm(has_many = "super::post::Entity")]
	Post
}

impl Related<super::post::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Post.def()
	}
}

impl ActiveModelBehavior for ActiveModel
{
}

pub struct RegisterUserModel
{
	pub username:      String,
	pub first_name:    String,
	pub last_name:     String,
	pub email:         String,
	pub date_of_birth: NaiveDate,
	pub password:      String
}
