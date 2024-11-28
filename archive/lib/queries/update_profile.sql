update "profile"
set
    user_id = $2,
    display_name = $3,
    avatar_url = $4,
    bio = $5,
    website_url = $6,
    twitter_username = $7,
    github_username = $8,
    status = $9,
    visibility = $10,
    is_deleted = $11,
    is_locked = $12,
    is_featured = $13,
    is_bot = $14,
    is_sponsor = $15,
    bluesky_url = $16,
    linkedin_url = $17,
    updated_at = NOW()
where id = $1
returning id;
