use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromJsonQueryResult)]
#[sea_orm(table_name = "posts")]
pub struct Model
{
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id:      i32,
	pub user_id: i32,
	pub title:   String,
	#[sea_orm(column_type = "Text")]
	pub text:    String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
	#[sea_orm(belongs_to = "super::user::Entity", from = "Column::UserId", to = "super::user::Column::Id")]
	User
}

impl Related<super::user::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel
{
}
