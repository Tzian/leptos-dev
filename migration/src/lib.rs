pub use sea_orm_migration::prelude::*;

mod m20240415_000001_create_post_table;
mod m20240416_000001_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
	fn migrations() -> Vec<Box<dyn MigrationTrait>>
	{
		vec![Box::new(m20240415_000001_create_post_table::Migration), Box::new(m20240416_000001_create_user_table::Migration)]
	}
}
