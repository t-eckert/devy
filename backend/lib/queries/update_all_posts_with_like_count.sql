UPDATE post
SET like_count = (
    SELECT COUNT(*)
    FROM "like"
    WHERE "like".post_id = post.id
);
