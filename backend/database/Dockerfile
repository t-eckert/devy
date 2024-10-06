FROM postgres:alpine

COPY ./backend/lib/migrations/ /docker-entrypoint-initdb.d/
COPY ./backend/database/seed/ /docker-entrypoint-initdb.d/

CMD ["postgres"]
