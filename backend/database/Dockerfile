FROM postgres:alpine

COPY ./lib/migrations/ /docker-entrypoint-initdb.d/
COPY ./database/seed/ /docker-entrypoint-initdb.d/

CMD ["postgres"]
