use uuid::Uuid;
use crate::entities::Blog;
use crate::db::{Error,Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation {
    pub id: Option<Uuid>,
    pub profile_id: Option<Uuid>,
    pub user_id: Option<Uuid>,

    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl TryFrom<Relation> for Blog {
    type Error = Error;

    fn try_from(r: Relation) -> Result<Blog> {
        Ok(Blog{
            id: r.id.ok_or(Error::missing_field("id"))?,
            profile_id: r.profile_id.ok_or(Error::missing_field("profile_id"))?,
            user_id: r.user_id.ok_or(Error::missing_field("user_id"))?,

            name: r.name.ok_or(Error::missing_field("name"))?,
            slug: r.slug.ok_or(Error::missing_field("slug"))?,
            description: r.description,

            created_at: r.created_at,
            updated_at: r.updated_at,
        })
    }
}
