pub mod mutation;
pub mod query;
pub mod utils;

pub use sea_orm;

pub use crate::{mutation::{post::PostMutation, user::UserMutation},
                query::{post::PostQuery, user::UserQuery}};
