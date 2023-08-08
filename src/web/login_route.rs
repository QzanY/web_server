use axum::{Router, Json, routing::post, extract::State};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{user::DataBase, error::Error};


pub fn routes(db:DataBase) -> Router
{
    Router::new().route("/api/login", post(api_login)).with_state(db)
}

async fn api_login(State(db):State<DataBase>,payload: Json<LoginCredits>) -> Result<Json<Value>,Error>
{
    println!(">> Someone trying to log in!");

    let check = db.search_user(&payload.name).await;
    
    if !(check)
    {
        return Err(Error::LoginFailNoSuchUserExists);
    }

    if !(db.check_password(&payload.name, &payload.password).await)
    {
        return Err(Error::LoginFailWrongPassword);
    }

    println!(">> Someone logged in!");

    let res = Json(json!(
        {
            "result":{
                "success": true
            }
        }
    ));
    Ok(res)

}

#[derive(Deserialize)]
struct LoginCredits
{
    name: String,
    password: String,
}