use actix_web::{http::StatusCode, post, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{NewUser, User};
use crate::schema::users;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub user_key: String,
    pub salt: String,
    pub iterations: i32,
    pub vaultiv: String,
    pub vault: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: UserResponse,
    pub vault: String,
    pub iterations: i32,
    pub vaultiv: String,
    pub salt: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error_msg: String,
    code: &'static str,
}

fn error_response(status: StatusCode, error_msg: &str, code: &'static str) -> HttpResponse {
    HttpResponse::build(status).json(ErrorResponse {
        error_msg: error_msg.to_string(),
        code,
    })
}

#[post("/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    payload: web::Json<RegisterRequest>,
) -> HttpResponse {
    let request = payload.into_inner();
    if request.email.trim().is_empty() || request.user_key.trim().is_empty() || request.salt.trim().is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "email and user_key and salt are required",
            "INVALID_INPUT",
        );
    }

    let new_user: NewUser = NewUser {
        id: Uuid::new_v4(),
        email: request.email,
        user_key: request.user_key,
        salt: request.salt,
        vault: request.vault,
        iterations: request.iterations,
        vaultiv: request.vaultiv,
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database unavailable",
                "DB_ERROR",
            )
        }
    };

    let inserted: User = match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
    {
        Ok(inserted) => inserted,
        Err(e) => {
            log::error!("failed to create user: {:?}", e);
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to create user",
                "DB_ERROR",
            )
        }
    };

    let response = RegisterResponse {
        user: UserResponse {
            id: inserted.id,
            email: inserted.email,
        },
        vault: inserted.vault,
        salt: inserted.salt,
        iterations: inserted.iterations,
        vaultiv: inserted.vaultiv,
    };

    HttpResponse::Created().json(response)
}
