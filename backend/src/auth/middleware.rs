use axum::{
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};

use crate::{auth::jwt, error::ApiError, state::AppState, types::Role};

#[derive(Debug, Clone)]
pub struct Principal {
    pub id: String,
    pub role: Role,
}

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let principal = principal_from_request(&state, &request)?;
    request.extensions_mut().insert(principal);
    Ok(next.run(request).await)
}

pub async fn require_admin(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let principal = principal_from_request(&state, &request)?;
    if principal.role != Role::Admin {
        return Err(ApiError::forbidden("Admin access required."));
    }

    request.extensions_mut().insert(principal);
    Ok(next.run(request).await)
}

impl<S> FromRequestParts<S> for Principal
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Principal>()
            .cloned()
            .ok_or_else(|| ApiError::unauthorized("Authentication required."))
    }
}

fn principal_from_request(state: &AppState, request: &Request) -> Result<Principal, ApiError> {
    let token = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| ApiError::unauthorized("Missing bearer token."))?;
    let claims = jwt::decode_token(&state.jwt_secret, token)?;
    let role =
        Role::from_db(&claims.role).ok_or_else(|| ApiError::unauthorized("Invalid token role."))?;

    Ok(Principal {
        id: claims.sub,
        role,
    })
}