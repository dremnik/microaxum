use axum::Extension;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{delete, get, patch, post};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use validator::Validate;

use crate::app::AppState;
use crate::auth::UserContext;
use crate::db::models::users::{NewUser, User, UserRecord, UserUpdate, UserUpdateRecord};
use crate::db::record::IntoNewRecord;
use crate::error::{bad_request_error, database_error, internal_error};

/// ==============================================
///        INDEX :: `/api/users/route.rs`
/// ----------------------------------------------
///
///   list_users()   ::     GET   /users
///   create_user()  ::    POST   /users
///   get_user()     ::     GET   /users/{id}
///   update_user()  ::   PATCH   /users/{id}
///   delete_user()  ::  DELETE   /users/{id}
///
/// ==============================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/{id}", get(get_user))
        .route("/{id}", patch(update_user))
        .route("/{id}", delete(delete_user))
}

/// - GET /users -
///
/// Returns a list of all users.
pub async fn list_users(
    State(app_state): State<AppState>,
    Extension(_current_user): Extension<UserContext>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;

    let mut conn = app_state.db_pool.get().await.map_err(internal_error)?;
    let records: Vec<UserRecord> = users.load(&mut conn).await.map_err(database_error)?;
    let data: Vec<User> = records.into_iter().map(|record| record.into()).collect();
    Ok(Json(data))
}

/// GET /users/{id}
///
/// Returns a single user by ID.
pub async fn get_user(
    State(app_state): State<AppState>,
    Extension(_current_user): Extension<UserContext>,
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    use crate::schema::users::dsl::{id as user_id, users};

    let mut conn = app_state.db_pool.get().await.map_err(internal_error)?;
    let record: UserRecord = users
        .filter(user_id.eq(id))
        .first(&mut conn)
        .await
        .map_err(database_error)?;

    let user: User = record.into();
    Ok(Json(user))
}

/// POST /users
///
/// Handles the creation of a new user.
pub async fn create_user(
    State(app_state): State<AppState>,
    Extension(_current_user): Extension<UserContext>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;

    new_user.validate().map_err(bad_request_error)?;

    let mut conn = app_state.db_pool.get().await.map_err(internal_error)?;

    let record: UserRecord = new_user.into_new_record();
    let record = diesel::insert_into(users)
        .values(record)
        .get_result::<UserRecord>(&mut conn)
        .await
        .map_err(database_error)?;

    let user: User = record.into();
    Ok(Json(user))
}

/// PATCH /users/{id}
///
/// Updates an existing user.
pub async fn update_user(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Json(update): Json<UserUpdate>,
) -> Result<Json<User>, (StatusCode, String)> {
    use crate::schema::users::dsl::{id as user_id, users};

    update.validate().map_err(bad_request_error)?;
    let update: UserUpdateRecord = update.into();

    let mut conn = app_state.db_pool.get().await.map_err(internal_error)?;
    let record = diesel::update(users)
        .filter(user_id.eq(id))
        .set(&update)
        .returning(users::all_columns())
        .get_result::<UserRecord>(&mut conn)
        .await
        .map_err(database_error)?;

    let user: User = record.into();
    Ok(Json(user))
}

/// DELETE /users/{id}
///
/// Soft deletes a user by setting its state to Deleted and returns the updated user data.
pub async fn delete_user(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    use crate::schema::users::dsl::{id as user_id, users};

    let mut conn = app_state.db_pool.get().await.map_err(internal_error)?;

    // Get the user record before deleting it so we can return it
    let record = users
        .filter(user_id.eq(&id))
        .first::<UserRecord>(&mut conn)
        .await
        .map_err(database_error)?;

    // Perform the actual deletion
    diesel::delete(users.filter(user_id.eq(id)))
        .execute(&mut conn)
        .await
        .map_err(database_error)?;

    let user: User = record.into();
    Ok(Json(user))
}
