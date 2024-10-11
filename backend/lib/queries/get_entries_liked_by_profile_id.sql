select
 	post.id as id,
	blog.slug as blog_slug,
	blog.name as blog_name,
	post.slug as slug,
	"user".username as author_slug,
	profile.display_name as author_name,
	post.created_at,
	post.updated_at,
	title,
	like_count,
	is_draft
from "like"
left join "post" on "like".post_id = "post".id
left join "blog" on "post".blog_id = "blog".id
left join "profile" on "blog".profile_id = "profile".id
left join "user" on "profile".user_id = "user".id
where "like".profile_id = $1
order by "like".created_at DESC;
