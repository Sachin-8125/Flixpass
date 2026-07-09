use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Customer,
    Admin,
}

impl Role {
    pub fn as_db(self) -> &'static str {
        match self {
            Self::Customer => "CUSTOMER",
            Self::Admin => "ADMIN",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "CUSTOMER" => Some(Self::Customer),
            "ADMIN" => Some(Self::Admin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingStatus {
    Confirmed,
    Cancelled,
}

impl BookingStatus {
    pub fn as_db(self) -> &'static str {
        match self {
            Self::Confirmed => "CONFIRMED",
            Self::Cancelled => "CANCELLED",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "CONFIRMED" => Some(Self::Confirmed),
            "CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}