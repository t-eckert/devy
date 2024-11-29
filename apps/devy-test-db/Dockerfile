FROM postgres:alpine

COPY ./apps/devy-test-db/dbs.sql /docker-entrypoint-initdb.d/

CMD ["postgres"]
