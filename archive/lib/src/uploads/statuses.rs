use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::Display;

#[derive(Display, Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Status {
    PENDING,
    REJECTED,

    VERIFIED,
    RECEIVED,
    CLONED,
    DIFFED,
    COMMITTED,
    SYNCED,

    DONE,
    FAILED,

    UNKNOWN,
}

impl From<std::string::String> for Status {
    fn from(s: std::string::String) -> Self {
        match s.as_str() {
            "pending" => Status::PENDING,
            "rejected" => Status::REJECTED,
            "verified" => Status::VERIFIED,
            "received" => Status::RECEIVED,
            "cloned" => Status::CLONED,
            "diffed" => Status::DIFFED,
            "committed" => Status::COMMITTED,
            "synced" => Status::SYNCED,
            "done" => Status::DONE,
            "failed" => Status::FAILED,
            _ => Status::UNKNOWN,
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "pending" => Status::PENDING,
            "rejected" => Status::REJECTED,
            "verified" => Status::VERIFIED,
            "received" => Status::RECEIVED,
            "cloned" => Status::CLONED,
            "diffed" => Status::DIFFED,
            "committed" => Status::COMMITTED,
            "synced" => Status::SYNCED,
            "done" => Status::DONE,
            "failed" => Status::FAILED,
            _ => Status::UNKNOWN,
        }
    }
}
