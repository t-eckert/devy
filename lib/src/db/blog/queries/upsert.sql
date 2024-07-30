-- $1: profile_id
-- $2: name
-- $3: slug
-- $4: description

WITH inserted_blog AS (
    INSERT INTO "blog" (profile_id, name, slug, description)
    VALUES (
        $1, $2, $3, $4
    )
    ON CONFLICT (slug) DO UPDATE SET
        profile_id = $1,
        name = $2,
        description = $4,
        updated_at = NOW()
    RETURNING
        id,
        profile_id,
        name,
        slug,
        description,
        created_at,
        updated_at
)
SELECT
    ib.id,
    ib.profile_id,
    p.user_id,
    ib.name,
    ib.slug,
    ib.description,
    to_char(ib.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
    to_char(ib.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM
    inserted_blog ib
    LEFT JOIN "profile" p ON ib.profile_id = p.id;
