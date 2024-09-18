select profile_id, post_id, "like".created_at
from "like"
left join "profile" on "like".profile_id = "profile".id
left join "user" on "profile".user_id = "user".id
where username = $1;
