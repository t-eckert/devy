use core::fmt;
use std::{fmt::Display, str::FromStr};

use crate::{date::Date, github};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Profile represents a user's profile.
/// A user can have more than one profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The unique identifier for the profile.
    pub id: Uuid,
    /// The unique identifier for the user that owns the profile.
    pub user_id: Uuid,

    /// The display name of the profile.
    pub display_name: Option<String>,
    /// The URL to the avatar image of the profile.
    pub avatar_url: Option<String>,
    /// The biography of the profile.
    pub bio: Option<String>,
    /// The URL to the website of the profile.
    pub website_url: Option<String>,
    /// The username to the Twitter profile of the profile.
    pub twitter_username: Option<String>,
    /// The username to the GitHub profile of the profile.
    pub github_username: Option<String>,
    /// The URL to the Bluesky profile of the profile.
    pub bluesky_url: Option<String>,
    /// The URL to the LinkedIn profile of the profile.
    pub linkedin_url: Option<String>,

    /// The status of the profile.
    pub status: Status,
    /// The visibility of the profile.
    pub visibility: Visibility,

    /// Whether the profile is deleted.
    pub is_deleted: bool,
    /// Whether the profile is locked.
    pub is_locked: bool,
    /// Whether the profile is featured.
    pub is_featured: bool,
    /// Whether the profile is a bot.
    pub is_bot: bool,
    /// Whether the profile is a sponsor.
    pub is_sponsor: bool,

    /// When the profile was first created.
    pub created_at: Date,
    /// When the profile was last updated.
    pub updated_at: Date,
}

/// Status represents the status of a profile.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Active,
    Suspended,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Suspended => write!(f, "suspended"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            _ => Err(format!("unknown status: {}", s)),
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "active" => Self::Active,
            "suspended" => Self::Suspended,
            _ => panic!("unknown status: {}", s),
        }
    }
}

/// Visibility represents the visibility of a profile.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    Public,
    Private,
    Unlisted,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Unlisted => write!(f, "unlisted"),
        }
    }
}

impl FromStr for Visibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            "unlisted" => Ok(Self::Unlisted),
            _ => Err(format!("unknown visibility: {}", s)),
        }
    }
}

impl From<String> for Visibility {
    fn from(s: String) -> Self {
        match s.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            "unlisted" => Self::Unlisted,
            _ => panic!("unknown visibility: {}", s),
        }
    }
}

impl Profile {
    /// Create a new profile with the given user ID.
    pub fn new(user_id: Uuid, display_name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            display_name: Some(display_name.to_string()),
            avatar_url: None,
            bio: None,
            website_url: None,
            twitter_username: None,
            github_username: None,
            bluesky_url: None,
            linkedin_url: None,
            status: Status::Active,
            visibility: Visibility::Public,
            is_deleted: false,
            is_locked: false,
            is_featured: false,
            is_bot: false,
            is_sponsor: false,
            created_at: Date::now(),
            updated_at: Date::now(),
        }
    }

    /// Update the profile using information from GitHub.
    pub fn update_from_github_user(&mut self, github_user: github::GitHubUser) {
        self.display_name = github_user.name;
        self.avatar_url = github_user.avatar_url;
        self.bio = github_user.bio;
        self.website_url = github_user.blog;
        self.twitter_username = github_user.twitter_username;
        self.github_username = github_user.login;
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Profile {{ id: {}, user_id: {}, display_name: {:?}, avatar_url: {:?}, bio: {:?}, website_url: {:?}, twitter_username: {:?}, github_username: {:?}, bluesky_url: {:?}, linkedin_url: {:?}, status: {}, visibility: {}, is_deleted: {}, is_locked: {}, is_featured: {}, is_bot: {}, is_sponsor: {}, created_at: {}, updated_at: {} }}",
            self.id,
            self.user_id,
            self.display_name,
            self.avatar_url,
            self.bio,
            self.website_url,
            self.twitter_username,
            self.github_username,
            self.bluesky_url,
            self.linkedin_url,
            self.status,
            self.visibility,
            self.is_deleted,
            self.is_locked,
            self.is_featured,
            self.is_bot,
            self.is_sponsor,
            self.created_at,
            self.updated_at,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_profile() {
        let user_id = Uuid::new_v4();
        let profile = Profile::new(user_id, "Alice O'Connor");
        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.display_name, Some("Alice O'Connor".to_string()));
        assert_eq!(profile.avatar_url, None);
        assert_eq!(profile.bio, None);
        assert_eq!(profile.website_url, None);
        assert_eq!(profile.twitter_username, None);
        assert_eq!(profile.github_username, None);
        assert_eq!(profile.bluesky_url, None);
        assert_eq!(profile.linkedin_url, None);
        assert_eq!(profile.status, Status::Active);
        assert_eq!(profile.visibility, Visibility::Public);
        assert_eq!(profile.is_deleted, false);
        assert_eq!(profile.is_locked, false);
        assert_eq!(profile.is_featured, false);
        assert_eq!(profile.is_bot, false);
        assert_eq!(profile.is_sponsor, false);
        assert!(profile.created_at <= Date::now());
        assert!(profile.updated_at <= Date::now());
    }
}
