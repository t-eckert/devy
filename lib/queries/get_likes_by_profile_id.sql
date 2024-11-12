select profile_id, post_id, created_at
from "like"
where profile_id = $1;
