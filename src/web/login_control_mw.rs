use axum::{response::Response, http::Request, middleware::Next};
use tower_cookies::Cookies;
use crate::{Error, user};

use super::AUTH_TOKEN;


pub async fn login_checker<T>(
    cookies: Cookies,
    req: Request<T>,
    next: Next<T>
) -> Result<Response,Error>
{
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let (username, expire, sign) = auth_token.ok_or(Error::AuthErrorNoTokenCookie).and_then(parse_token).unwrap();

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(String,String,String),Error>
{
    let mut parts = token.split(".");
    let username:String = parts.next().unwrap().to_string();
    let expire:String = parts.next().unwrap().to_string();
    let sign:String = parts.next().unwrap().to_string();
    Ok((username,expire,sign))
    
    
}