use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{
        Request, StatusCode,
        header::{AUTHORIZATION, HeaderMap, HeaderValue, WWW_AUTHENTICATE},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose};

use super::{config::AuthSettings, state::AppState};

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    if !has_valid_basic_auth(request.headers(), &state.auth) {
        return Err(unauthorized_response());
    }

    Ok(next.run(request).await)
}

pub async fn require_https(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    if should_require_https(&state.auth, request.headers()) {
        return Err((StatusCode::FORBIDDEN, "https is required").into_response());
    }

    Ok(next.run(request).await)
}

fn should_require_https(auth: &AuthSettings, headers: &HeaderMap) -> bool {
    auth.require_https && !auth.development && !is_https(headers) && !is_local_request(headers)
}

fn is_https(headers: &HeaderMap) -> bool {
    headers
        .get("x-forwarded-proto")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value
                .split(',')
                .next()
                .is_some_and(|proto| proto.trim().eq_ignore_ascii_case("https"))
        })
}

fn is_local_request(headers: &HeaderMap) -> bool {
    ["host", "x-forwarded-host"].iter().any(|header| {
        headers
            .get(*header)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| is_local_host(value))
    })
}

fn is_local_host(value: &str) -> bool {
    let value = value.split(',').next().unwrap_or(value).trim();
    let host = if let Some(rest) = value.strip_prefix('[') {
        rest.split(']').next().unwrap_or(rest)
    } else if value.matches(':').count() == 1 {
        value.split(':').next().unwrap_or(value)
    } else {
        value
    };

    matches!(host, "localhost" | "127.0.0.1" | "::1")
}

fn has_valid_basic_auth(headers: &HeaderMap, auth: &AuthSettings) -> bool {
    let Some(encoded) = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Basic "))
    else {
        return false;
    };

    let Ok(decoded) = general_purpose::STANDARD.decode(encoded) else {
        return false;
    };
    let Ok(credentials) = String::from_utf8(decoded) else {
        return false;
    };
    let Some((username, password)) = credentials.split_once(':') else {
        return false;
    };

    username == auth.username && password == auth.password
}

fn unauthorized_response() -> Response {
    let mut response = (StatusCode::UNAUTHORIZED, "authentication required").into_response();
    response.headers_mut().insert(
        WWW_AUTHENTICATE,
        HeaderValue::from_static(r#"Basic realm="summit26-registration""#),
    );
    response
}
