#![allow(unused)]
use std::f32::consts::E;

use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{async_trait, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{CustomError, Result};

pub async fn mw_require_auth<B>(
    // ctx: Ctx,
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    ctx?;

    // Parse token.
    // let (user_id, exp, sign) = auth_token
    //     .ok_or(CustomError::AuthFailNoAuthTokenCookie)
    //     .and_then(parse_token)?;

    // TODO: Token components validation.

    Ok(next.run(req).await)
}

pub async fn my_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - my_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(CustomError::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validation.
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(CustomError::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// region:     --- Aux Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(CustomError::AuthFailCtxNotInRequestExt)?
            .clone()

        // // User the cookies extractor.
        // let cookies = parts.extract::<Cookies>().await.unwrap();

        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // // Parse token.
        // let (user_id, exp, sign) = auth_token
        //     .ok_or(CustomError::AuthFailNoAuthTokenCookie)
        //     .and_then(parse_token)?;

        // // TODO: Token components validation.

        // Ok(Ctx::new(user_id))
    }
}
// endregion:  --- Aux Extractor

// Parse a token of format `user-[user-id].[expiration].[signature]`
// Returns (user-id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(CustomError::AuthFailTokenWrongFormat)?;

    let user_id = user_id
        .parse()
        .map_err(|_| CustomError::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
