# BUILDER
FROM rust:1.71-slim-buster as builder

RUN mkdir /app
COPY . /app
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN SQLX_OFFLINE=true cargo build --release

# RUNNER
FROM debian:buster-slim

RUN mkdir /app
COPY . /app
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY --from=builder /app/target/release/api .
COPY --from=builder /app/Rocket.toml .

ENV DATABASE_URL=
ENV CLIENT_ID=
ENV CLIENT_SECRET=
ENV POST_AUTH_REDIRECT=
ENV ROCKET_DATABASES=

EXPOSE 8080
CMD ["./api"]
