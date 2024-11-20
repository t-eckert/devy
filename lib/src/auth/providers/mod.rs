use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub enum Provider {
    GitHub,
}
