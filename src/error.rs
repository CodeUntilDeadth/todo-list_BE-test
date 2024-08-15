use axum::{http::StatusCode, response::IntoResponse};
use oauth2::{basic::BasicErrorResponseType, reqwest, RequestTokenError, StandardErrorResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Unauthorized { message: String },
    OAuthError(String),
    Other(anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Unauthorized { message } => (StatusCode::BAD_REQUEST, message),
            Error::OAuthError(message) => (StatusCode::BAD_REQUEST, message),
            Error::Other(err) => (StatusCode::BAD_REQUEST, err.to_string()),
        }
        .into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Other(value)
    }
}

impl<T> From<RequestTokenError<reqwest::Error<T>, StandardErrorResponse<BasicErrorResponseType>>>
    for Error
where
    T: std::error::Error + 'static,
{
    fn from(
        value: RequestTokenError<reqwest::Error<T>, StandardErrorResponse<BasicErrorResponseType>>,
    ) -> Self {
        Error::Unauthorized {
            message: value.to_string(),
        }
    }
}

impl From<oauth2::url::ParseError> for Error {
    fn from(value: oauth2::url::ParseError) -> Self {
        Error::Other(value.into())
    }
}

impl From<envy::Error> for Error {
    fn from(value: envy::Error) -> Self {
        Error::Other(value.into())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::OAuthError(value)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::Other(value.into())
    }
}
