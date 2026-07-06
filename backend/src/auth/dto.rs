use serde::{Deserialize, Serialize};
use crate::types::Role;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest{
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest{
    pub email: String, 
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct AuthResponse{
    pub token: String,
    pub user: UserDto
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
}