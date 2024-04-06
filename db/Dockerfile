FROM postgres:alpine

COPY ./db/migrations/ /docker-entrypoint-initdb.d/
COPY ./db/seed/ /docker-entrypoint-initdb.d/

CMD ["postgres"]
