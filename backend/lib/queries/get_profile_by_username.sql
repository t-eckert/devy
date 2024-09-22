select
    profile.id,
    user_id,
    display_name,
    profile.avatar_url,
    profile.bio,
    profile.website_url,
    profile.twitter_username,
    profile.github_username,
    profile.status,
    profile.visibility,
    profile.is_deleted,
    profile.is_locked,
    profile.is_featured,
    profile.is_bot,
    profile.is_sponsor,
    profile.created_at,
    profile.updated_at
from "profile"
left join "user" on "profile".user_id = "user".id
where "user".username = $1;
