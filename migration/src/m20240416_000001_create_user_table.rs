use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.create_table(Table::create().table(Users::Table)
		                                    .if_not_exists()
		                                    .col(ColumnDef::new(Users::Id).integer().not_null().auto_increment().primary_key())
		                                    .col(ColumnDef::new(Users::Username).string().unique_key().not_null())
		                                    .col(ColumnDef::new(Users::FirstName).string().not_null())
		                                    .col(ColumnDef::new(Users::LastName).string().not_null())
		                                    .col(ColumnDef::new(Users::Email).string().unique_key().not_null())
		                                    .col(ColumnDef::new(Users::DateOfBirth).date().not_null())
		                                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
		                                    .col(ColumnDef::new(Users::Uuid).uuid().unique_key().not_null())
		                                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
		                                    .col(ColumnDef::new(Users::UpdatedAt).date_time().not_null())
		                                    .to_owned())
		       .await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
	}
}

#[derive(DeriveIden)]
pub enum Users
{
	Table,
	Id,
	Username,
	FirstName,
	LastName,
	Email,
	DateOfBirth,
	PasswordHash,
	Uuid,
	CreatedAt,
	UpdatedAt
}
