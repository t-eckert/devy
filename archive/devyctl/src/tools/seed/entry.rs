use lib::openai;
use serde::{Deserialize, Serialize};

use super::Record;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub user: UserEntry,
    pub profile: ProfileEntry,
    pub blog: BlogEntry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEntry {
    pub username: String,
    pub email: String,
    pub github_username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileEntry {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogEntry {
    pub name: String,
    pub subject: String,
    pub posts: Vec<PostEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostEntry {
    pub slug: String,
    pub title: String,
    pub content: String,
}

impl Entry {
    pub async fn generate(
        openai_client: openai::Client,
        record: Record,
    ) -> Result<Self, anyhow::Error> {
        Ok(Entry {
            user: generate_user_entry(&openai_client, &record).await?,
            profile: generate_profile_entry(&openai_client, &record).await?,
            blog: generate_blog_entry(&openai_client, &record).await?,
        })
    }
}

async fn generate_user_entry(
    client: &openai::Client,
    record: &Record,
) -> Result<UserEntry, anyhow::Error> {
    let username = client.query(format!("Given the name, {} generate a username that is unique using only alphanumeric characters.", record.name)).await?;
    let email = client
        .query(format!(
            "Given the username, {} generate an email address that is unique.",
            username
        ))
        .await?;
    let github_username = client.query(format!("Given the name, {} generate a GitHub username that is unique using only alphanumeric characters.", record.name)).await?;

    Ok(UserEntry {
        username,
        email,
        github_username,
    })
}

async fn generate_profile_entry(
    client: &openai::Client,
    record: &Record,
) -> Result<ProfileEntry, anyhow::Error> {
    let description = client
        .query(format!(
            "Given the name, {}, and the fact that this person writes about {} generate a description of their profile that is unique.",
            record.name, record.blog_subject
        ))
        .await?;

    Ok(ProfileEntry {
        name: record.name.clone(),
        description,
    })
}

async fn generate_blog_entry(
    client: &openai::Client,
    record: &Record,
) -> Result<BlogEntry, anyhow::Error> {
    let mut posts = vec![];
    for _ in 0..record.post_count {
        posts.push(generate_post_entry(&client, &record).await?);
    }

    Ok(BlogEntry {
        name: record.blog_name.clone(),
        subject: record.blog_subject.clone(),
        posts,
    })
}

async fn generate_post_entry(
    client: &openai::Client,
    record: &Record,
) -> Result<PostEntry, anyhow::Error> {
    let title = client
        .query(format!(
            "Given the name, {}, and the fact that this person writes about {} generate a title for a blog post that is unique. Don't put any quotes around the title.",
            record.name, record.blog_subject
        ))
        .await?.replace('"', "");
    let slug = lib::slug(&title);
    let content = client
        .query(format!(
            "Given the title, {} and the fact that this person writes about {} generate a blog post that is between 3 and 10 paragraphs formatted in markdown. If appropriate, feel free to include code examples, tables, and images.",
            title, record.blog_subject
        ))
        .await?;

    Ok(PostEntry {
        slug,
        title,
        content,
    })
}
