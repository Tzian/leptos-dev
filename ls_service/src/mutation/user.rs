use chrono::Utc;
use ls_entity::{user, user::Entity as User};
use sea_orm::*;
use sha256::digest;
use uuid::Uuid;

pub struct UserMutation;

impl UserMutation
{
    pub async fn create_new_user(db: &DbConn,
                                 form_data: user::RegisterUserModel)
                                 -> Result<user::ActiveModel, DbErr>
    {
        user::ActiveModel { username: Set(form_data.username.to_owned()),
                            first_name: Set(form_data.first_name.to_owned()),
                            last_name: Set(form_data.last_name.to_owned()),
                            email: Set(form_data.email.to_owned()),
                            password_hash: Set(digest(form_data.password.to_owned())),
                            date_of_birth: Set(form_data.date_of_birth.to_owned()),
                            uuid: Set(Uuid::new_v4()),
                            created_at: Set(Utc::now().naive_utc()),
                            updated_at: Set(Utc::now().naive_utc()),
                            ..Default::default() }.save(db)
                                                  .await
    }

    pub async fn update_user_by_id(db: &DbConn,
                                   id: i32,
                                   form_data: user::Model)
                                   -> Result<user::Model, DbErr>
    {
        let user: user::ActiveModel =
            User::find_by_id(id).one(db)
                                .await?
                                .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
                                .map(Into::into)?;

        user::ActiveModel { id:            user.id,
                            username:      Set(form_data.username.to_owned()),
                            first_name:    Set(form_data.first_name.to_owned()),
                            last_name:     Set(form_data.last_name.to_owned()),
                            email:         Set(form_data.email.to_owned()),
                            password_hash: Set(digest(form_data.password_hash.to_owned())),
                            date_of_birth: Set(form_data.date_of_birth.to_owned()),
                            uuid:          user.uuid,
                            created_at:    user.created_at,
                            updated_at:    Set(Utc::now().naive_utc()) }.update(db)
                                                                        .await
    }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr>
    {
        let user: user::ActiveModel =
            User::find_by_id(id).one(db)
                                .await?
                                .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
                                .map(Into::into)?;

        user.delete(db).await
    }
}
