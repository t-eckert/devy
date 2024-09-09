insert into "like" (profile_id, post_id)
values ($1, $2)
    on conflict (profile_id, post_id)
    do nothing;
