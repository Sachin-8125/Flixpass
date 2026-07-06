use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::{
    auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest, UserDto},
        jwt, password,
    },
    db::models::{NewUser, UserRow},
    error::ApiError,
    repositories::users,
    state::AppState,
    types::Role,
};

pub async fn register(state: AppState, input: RegisterRequest) -> Result<AuthResponse, ApiError> {
    if input.password.len() < 8 {
        return Err(ApiError::bad_request(
            "Password must be at least 8 characters.",
        ));
    }

    let password_hash = password::hash_password(&input.password)?;
    let jwt_secret = state.jwt_secret.clone();
    let pool = state.db.clone();

    let created = tokio::task::spawn_blocking(move || -> Result<UserRow, ApiError> {
        let mut conn = pool.get()?;
        let new_user = NewUser {
            id: uuid::Uuid::new_v4().to_string(),
            name: input.name,
            email: input.email.to_lowercase(),
            password_hash,
            role: Role::Customer.as_db().to_string(),
        };

        users::create(&mut conn, &new_user).map_err(|error| match error {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                ApiError::bad_request("Email is already registered.")
            }
            other => ApiError::from(other),
        })
    })
    .await
    .map_err(ApiError::from)??;

    build_auth_response(&jwt_secret, created)
}

pub async fn login(state: AppState, input: LoginRequest) -> Result<AuthResponse, ApiError> {
    let jwt_secret = state.jwt_secret.clone();
    let pool = state.db.clone();

    let found = tokio::task::spawn_blocking(move || -> Result<UserRow, ApiError> {
        let mut conn = pool.get()?;
        users::find_by_email(&mut conn, &input.email.to_lowercase())?
            .ok_or_else(|| ApiError::unauthorized("Invalid email or password."))
    })
    .await
    .map_err(ApiError::from)??;

    password::verify_password(&input.password, &found.password_hash)?;
    build_auth_response(&jwt_secret, found)
}

fn build_auth_response(jwt_secret: &str, user: UserRow) -> Result<AuthResponse, ApiError> {
    let role = Role::from_db(&user.role)
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("unknown user role in database")))?;
    let token = jwt::issue_token(jwt_secret, &user.id, &user.email, role)?;

    Ok(AuthResponse {
        token,
        user: UserDto {
            id: user.id,
            name: user.name,
            email: user.email,
            role,
        },
    })
}
