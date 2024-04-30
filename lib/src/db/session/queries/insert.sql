INSERT INTO "session" (user_id, metadata,  access_token, encoding_key)
VALUES ($1, $2, $3, $4)
RETURNING
    id,
    user_id,
    metadata,
    exp,
    access_token,
    encoding_key,
    to_char("session".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
    to_char("session".last_used_at, 'YYYY-MM-DDThh:mm:ss.ss') AS last_used_at;
