FROM postgres:alpine

COPY ./migrations/ /docker-entrypoint-initdb.d/
COPY ./seed/ /docker-entrypoint-initdb.d/

CMD ["postgres"]
