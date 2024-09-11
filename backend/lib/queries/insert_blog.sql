-- $1: profile_id
-- $2: name
-- $3: slug
-- $4: description

insert into "blog" (profile_id, name, slug, description)
values (
    $1, $2, $3, $4
) returning id;
