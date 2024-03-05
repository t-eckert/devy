FROM postgres:alpine

COPY ./crates/db/migrations/ /docker-entrypoint-initdb.d/
COPY ./crates/db/seed/ /docker-entrypoint-initdb.d/

CMD ["postgres"]
