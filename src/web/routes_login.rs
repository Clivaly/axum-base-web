use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{CustomError, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    print!("->> {:<12} - api_login\n", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "admin" || payload.pwd != "welcome" {
        return Err(CustomError::LoginFail);
    }

    // TODO: Set Cookies

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
