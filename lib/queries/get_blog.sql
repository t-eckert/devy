select
   "blog".id,
   "profile".id as profile_id,
   "user".id as user_id,

   "user".username as author_username,
   "profile".display_name as author_display_name,

   "blog".name as name,
   "blog".repo_url as repo_url,
   "blog".slug as slug,
   "blog".description as description,

   "blog".created_at,
   "blog".updated_at
from "blog"
left join "profile" on "blog".profile_id = "profile".id
left join "user" on "profile".user_id = "user".id
where "blog".id = $1;
