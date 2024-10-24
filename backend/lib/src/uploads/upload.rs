use crate::uploads::{Changeset, Status, UploadRepository};
use crate::{date::Date, db, git::Git};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub previous_upload_id: Option<Uuid>,

    pub status: Status,
    pub repo: String,
    pub sha: String,
    pub logs: Option<Vec<String>>,

    pub diff: Option<String>,
    pub changeset: Option<Changeset>,

    pub created_at: Date,
    pub updated_at: Date,
}

impl Upload {
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_sha(&mut self, sha: &str) {
        self.sha = sha.to_string();
    }

    pub fn set_diff(&mut self, diff: &str) {
        self.diff = Some(diff.to_string());
    }

    pub fn set_changeset(&mut self, changeset: Changeset) {
        self.changeset = Some(changeset);
    }

    pub fn append_log(&mut self, log: &str) {
        if let Some(logs) = &mut self.logs {
            logs.push(log.to_string());
        } else {
            self.logs = Some(vec![log.to_string()]);
        }
    }

    pub fn dir(&self) -> String {
        format!("/tmp/{}", self.id)
    }
}
