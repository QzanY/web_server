use axum::{Router, Json, routing::post, extract::State};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{user::DataBase, error::Error};


pub fn routes(db:DataBase) -> Router
{
    Router::new().route("/api/login", post(api_login)).with_state(db)
}

async fn api_login(
    cookies: Cookies,
    State(db):State<DataBase>,
    payload: Json<LoginCredits>) -> Result<Json<Value>,Error>
{
    println!(">> Someone is trying to log in!");

    let token = cookies.get(crate::web::AUTH_TOKEN).map(|c| c.value().to_string());
    match token
    {
        Some(_) => return Err(Error::LoginFailAlreadyLogged),
        None => {}
    }
    let check = db.search_user(&payload.name).await;
    
    if !(check)
    {
        return Err(Error::LoginFailNoSuchUserExists);
    }

    // TODO: Implement credential check using hashed values

    if !(db.check_password(&payload.name, &payload.password).await)
    {
        return Err(Error::LoginFailWrongPassword);
    }

    cookies.add(Cookie::build(crate::web::AUTH_TOKEN, format!("{}.exp.sign",payload.name)).path("/").finish());
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