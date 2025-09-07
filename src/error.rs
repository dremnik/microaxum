use axum::http::StatusCode;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: u16,
    pub description: String,
}

// (TODO): Add tracing to each of these for loggin

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::fmt::Display,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Utility function for database errors into appropriate responses
pub fn database_error(err: diesel::result::Error) -> (StatusCode, String) {
    match err {
        DieselError::NotFound => not_found_error(err),
        DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => {
            unprocessable_entity_error(err)
        }
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => conflict_error(err),
        _ => internal_error(err),
    }
}

/// Utility function for mapping any error into a `400 Bad Request`
/// response and serializing it into a JSON response.
pub fn bad_request_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    let res = ErrorResponse {
        code: StatusCode::BAD_REQUEST.as_u16(),
        description: err.to_string(),
    };

    let res = serde_json::to_string(&res)
        .unwrap_or_else(|_| "{\"code\":500,\"description\":\"Serialization error\"}".to_string());

    (StatusCode::BAD_REQUEST, res)
}

/// Utility function for mapping any error into a `404 Not Found`
/// response and serializing it into a JSON response.
pub fn not_found_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    let res = ErrorResponse {
        code: StatusCode::NOT_FOUND.as_u16(),
        description: err.to_string(),
    };

    let res = serde_json::to_string(&res)
        .unwrap_or_else(|_| "{\"code\":500,\"description\":\"Serialization error\"}".to_string());

    (StatusCode::NOT_FOUND, res)
}

/// Utility function for mapping any error into a `409 Unprocessable Entity`
/// response and serializing it into a JSON response.
pub fn conflict_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    let res = ErrorResponse {
        code: StatusCode::CONFLICT.as_u16(),
        description: err.to_string(),
    };

    let res = serde_json::to_string(&res)
        .unwrap_or_else(|_| "{\"code\":500,\"description\":\"Serialization error\"}".to_string());

    (StatusCode::CONFLICT, res)
}

/// Utility function for mapping any error into a `422 Unprocessable Entity`
/// response and serializing it into a JSON response.
pub fn unprocessable_entity_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    let res = ErrorResponse {
        code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
        description: err.to_string(),
    };

    let res = serde_json::to_string(&res)
        .unwrap_or_else(|_| "{\"code\":500,\"description\":\"Serialization error\"}".to_string());

    (StatusCode::UNPROCESSABLE_ENTITY, res)
}
