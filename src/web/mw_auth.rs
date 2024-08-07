use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{CustomError, Result};

pub async fn mw_require_auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-tokin parsing & validation.
    auth_token.ok_or(CustomError::AuthFailNoAuthTokenCookie)?;
    
    Ok(next.run(req).await)
}
