FROM postgres:alpine

COPY ./lib/migrations/ /docker-entrypoint-initdb.d/
COPY ./lib/seed/ /docker-entrypoint-initdb.d/

CMD ["postgres"]
