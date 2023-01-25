create policy "Enable read access for all users"
on "public"."post"
as permissive
for select
to public
using (true);



