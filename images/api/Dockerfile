FROM rust:1.76-slim-buster as builder
RUN mkdir /crates
COPY ./crates /crates
WORKDIR /crates
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN SQLX_OFFLINE=true cargo build --bin api --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev git
WORKDIR /app
COPY --from=builder /crates/target/release/api .
ENV DATABASE_URL=
ENV CLIENT_ID=
ENV CLIENT_SECRET=
ENV POST_AUTH_URI=
ENV GIT_PATH=/usr/bin/git
EXPOSE 8000
CMD ["./api"]
