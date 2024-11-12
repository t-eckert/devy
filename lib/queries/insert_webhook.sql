insert into "webhook" (type, payload)
values ($1, $2)
returning id;
