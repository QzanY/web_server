use serde::Serialize;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone, Debug)]
pub enum Error
{
    LoginFailWrongPassword,
    LoginFailNoSuchUserExists,
    LoginFailAlreadyLogged,

    RegisterErrorUserExists,

    DeleteUserError,

    AuthErrorNoTokenCookie,

}

impl core::fmt::Display for Error
{
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter
    ) -> core::result::Result<(),core::fmt::Error>
    {
        write!(fmt,"{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!(">> Error: {self}");

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(self);

		response
	}
}