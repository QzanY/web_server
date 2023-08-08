use axum::{Router, routing::{post,delete}, extract::State, Json};

use crate::{user::{DataBase, User}, error::Error};
use axum_macros::debug_handler;

pub fn routes(db: DataBase) -> Router
{
    Router::new()
    .route("/api/register", post(create_user).delete(delete_user))
    .with_state(db)
}
#[debug_handler]
async fn create_user(
    State(db): State<DataBase>,
    Json(new_user): Json<User>
) -> Result<Json<User>,Error>
{
    match db.create_user(new_user).await
    {
        Ok(value) => {
            println!(">> Creating new user");
            println!("{:?}",db);
            return Ok(Json(value))},
        Err(value) => {
            println!(">> Couldn't create new user");
            println!("{:?}",db);
            return Err(Error::RegisterErrorUserExists)}
    };

}

async fn delete_user(
    State(db): State<DataBase>,
    Json(old_user): Json<User>
) -> Result<Json<User>,Error>
{

    match db.delete_user(old_user.name()).await
    {
        Ok(value) => {
            println!(">> Deleting user");
            println!("{:?}",db);
            return Ok(Json(value))},
        Err(value) => {
            println!(">> Couldn't delete the user");
            println!("{:?}",db);
            return Err(Error::DeleteUserError)}
    }

}