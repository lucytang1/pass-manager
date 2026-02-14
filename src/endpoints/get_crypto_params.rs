use actix_web::{get, http::StatusCode, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::models::User;
use crate::schema::users;

#[derive(Deserialize)]
pub struct GetCryptoParamsRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct GetCryptoParamsResponse {
    pub salt: String,
    pub iterations: i32,
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

#[get("/get_crypto_params")]
pub async fn get_crypto_params(pool: web::Data<DbPool>, payload: web::Query<GetCryptoParamsRequest>) -> HttpResponse {
    let request = payload.into_inner();
    if request.email.trim().is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "email is required",
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
        .first(&mut conn)
    {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "user not found",
                "USER_NOT_FOUND",
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

    let response = GetCryptoParamsResponse { salt: user.salt, iterations: user.iterations };
    HttpResponse::Ok().json(response)
}
