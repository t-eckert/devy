use crate::tools::seed::entry::Entry;
use lib::{posts::Post, profile::Profile, user::User};

const AVATAR_URL: &str = "https://i.pravatar.cc/300";

pub fn format_entry(entry: Entry) -> String {
    let user = User::new(&entry.user.username, Some(&entry.user.email));
    let profile = Profile::new(user.id, &entry.profile.name);
    let blog = lib::blogs::Blog::new(&profile, &user, &entry.blog.name, "", "");

    let mut seed = String::new();
    seed.push_str(&format!("-- Entry: {}", entry.profile.name));
    seed.push_str(&format!(
        r#"
INSERT INTO "user"
    (id, username, email, github_username, role) VALUES
    ('{}', '{}', {}, {}, '{}');"#,
        user.id,
        user.username,
        user.email
            .map(|s| format!("'{}'", s))
            .unwrap_or("NULL".to_string()),
        user.github_username
            .map(|s| format!("'{}'", s))
            .unwrap_or("NULL".to_string()),
        user.role.to_string()
    ));
    seed.push_str(&format!(
        r#"
INSERT INTO "profile"
    (id, user_id, display_name, avatar_url, bio) VALUES
    ('{}', '{}', {}, '{}', {});"#,
        profile.id,
        profile.user_id,
        profile
            .display_name
            .map(|s| format!("'{}'", s))
            .unwrap_or("NULL".to_string()),
        AVATAR_URL,
        profile
            .bio
            .map(|s| format!("'{}'", s))
            .unwrap_or("NULL".to_string()),
    ));
    seed.push_str(&format!(
        r#"
INSERT INTO "blog"
    (id, profile_id, name, slug, description, repo_url) VALUES
    ('{}', '{}', '{}', '{}', {}, '{}');"#,
        blog.id,
        blog.profile_id,
        blog.name,
        blog.slug,
        blog.description
            .clone()
            .map(|s| format!("'{}'", s))
            .unwrap_or("NULL".to_string()),
        blog.repo_url,
    ));
    for post_entry in entry.blog.posts {
        let post = Post::new(
            &blog,
            &post_entry.slug,
            &post_entry.title,
            &post_entry.content,
        );
        seed.push_str(&format!(
            r#"
    INSERT INTO "post"
        (id, blog_id, title, slug, like_count, is_draft, body) VALUES
        ('{}', '{}', '{}', '{}', {}, {}, '{}');"#,
            post.id,
            post.blog_id,
            escape_string(&post.title),
            post.slug,
            post.like_count.unwrap_or(0),
            post.is_draft,
            escape_string(&post.body),
        ));
    }

    seed
}

fn escape_string(s: &str) -> String {
    s.replace("'", "''")
}
