select
    id,
    blog_id,
    url,
    metadata,
    created_at,
    updated_at
from "repo"
where url = $1;