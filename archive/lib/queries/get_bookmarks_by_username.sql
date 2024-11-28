select profile_id, post_id, "bookmark".created_at
from "bookmark"
left join "profile" on "bookmark".profile_id = "profile".id
left join "user" on "profile".user_id = "user".id
where username = $1;
