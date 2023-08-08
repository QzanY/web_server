#![allow(unused)]
use std::net::SocketAddr;

use axum::{ Router, extract::Query, response::{IntoResponse, Html}};
use axum::routing::get;
use tower_cookies::CookieManagerLayer;
use crate::error::Error;

mod web;
mod file;
mod user;
mod error;

#[tokio::main]
async fn main() -> Result<(),Error>
{

    let db = user::DataBase::new().await.unwrap();

    let main_route = Router::new()
        .merge(main_routes())
        .merge(web::login_route::routes(db.clone()))
        .merge(web::register_route::routes(db.clone()))
        .layer(CookieManagerLayer::new());
    
        

    let addr = SocketAddr::from(([127,0,0,1],10031));
    println!(">> SERVER STARTED LISTENING ON {addr} <<");

    axum::Server::bind(&addr).serve(main_route.into_make_service()).await.unwrap();

    Ok(())
}

fn main_routes() -> Router
{
    Router::new()
    .route("/",get(main_page))
    
}

// Query(vars): Query<Option<String>>
async fn main_page() -> Html<&'static str>
{
    println!(">> h1 deneme");
    include_str!("../pages/index.html").into()
}
