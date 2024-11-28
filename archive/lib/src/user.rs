use std::fmt::Display;

use crate::date::Date;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Represents a user of Devy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The unique identifier of the user.
    pub id: Uuid,

    /// The unique username of the user.
    pub username: String,
    /// The role of the user.
    pub role: Role,
    /// The status of the user.
    pub status: Status,

    /// The email address of the user.
    pub email: Option<String>,
    /// The GitHub username of the user.
    pub github_username: Option<String>,

    /// When the user was created.
    pub created_at: Date,
    /// When the user was last updated.
    pub updated_at: Date,
    /// When the user was last logged in.
    pub last_login_at: Option<Date>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Admin,
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Role::Admin => "Admin",
            Role::User => "User",
        };

        write!(f, "{}", role)
    }
}

impl From<Value> for Role {
    fn from(value: Value) -> Self {
        match value.as_str().unwrap() {
            "admin" => Role::Admin,
            "user" => Role::User,
            _ => panic!("Invalid role"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Active,
    Inactive,
    Suspended,
    Deleted,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Status::Active => "Active",
            Status::Inactive => "Inactive",
            Status::Suspended => "Suspended",
            Status::Deleted => "Deleted",
        };

        write!(f, "{}", status)
    }
}

impl From<Value> for Status {
    fn from(value: Value) -> Self {
        match value.as_str().unwrap() {
            "active" => Status::Active,
            "inactive" => Status::Inactive,
            "suspended" => Status::Suspended,
            "deleted" => Status::Deleted,
            _ => panic!("Invalid status"),
        }
    }
}

impl User {
    pub fn new(username: &str, email: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4(),
            username: username.to_string(),
            role: Role::User,
            status: Status::Active,
            email: email.map(|s| s.to_string()),
            github_username: None,
            created_at: Date::now(),
            updated_at: Date::now(),
            last_login_at: None,
        }
    }

    pub fn track_login(&mut self) {
        self.last_login_at = Some(Date::now());
    }

    pub fn with_github_username(mut self, github_username: &str) -> Self {
        self.github_username = Some(github_username.to_string());
        self
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let email = self.email.as_deref().unwrap_or("N/A");
        let last_login_at = self
            .last_login_at
            .map_or("N/A".to_string(), |d| d.to_string());

        write!(
            f,
            "User:\t{}\t({})\t{}\tlast login: {}",
            self.username, self.role, email, last_login_at
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new("alice", Some("alice@email.com"));

        assert_eq!(user.username, "alice");
        assert_eq!(user.role, Role::User);
        assert_eq!(user.status, Status::Active);
        assert_eq!(user.email, Some("alice@email.com".to_string()));
        assert_eq!(user.email, None);
        assert_eq!(user.github_username, None);
        assert_eq!(user.created_at, user.updated_at);
    }

    #[test]
    fn test_track_login() {
        let mut user = User::new("alice", None);
        user.track_login();

        assert!(user.last_login_at.unwrap().is_around(&Date::now(), 1));
    }
}
