use actix_web::{http::StatusCode, post, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::User;
use crate::schema::users;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub user_key: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub vault: String,
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

#[post("/auth")]
pub async fn auth(pool: web::Data<DbPool>, payload: web::Json<AuthRequest>) -> HttpResponse {
    let request = payload.into_inner();
    if request.email.trim().is_empty() || request.user_key.trim().is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "email and user_key are required",
            "INVALID_INPUT",
        );
    }

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

    let user: User = match users::table
        .filter(users::email.eq(&request.email))
        .filter(users::user_key.eq(&request.user_key))
        .first(&mut conn)
    {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "invalid email or user_key",
                "AUTH_FAILED",
            )
        }
        Err(e) => {
            log::error!("failed to fetch user: {:?}", e);
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch user",
                "DB_ERROR",
            );
        }
    };

    let response = AuthResponse {
        user: UserResponse {
            id: user.id,
            email: user.email,
        },
        vault: user.vault,
        salt: user.salt,
    };

    HttpResponse::Ok().json(response)
}
