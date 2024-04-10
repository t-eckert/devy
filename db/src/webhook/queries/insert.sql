-- $1: webhook_type
-- $2: payload

insert into webhook (type, payload)
values ($1, $2)
returning
    id,
    to_char(received_at, 'YYYY-MM-DDThh:mm:ss.ss') as received_at,
    type as webhook_type,
    payload;
