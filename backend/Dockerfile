FROM rust:1.71-slim-buster as builder
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN SQLX_OFFLINE=true cargo build --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev git
COPY --from=builder /app/target/release/devy-backend .
ENV DATABASE_URL=
ENV CLIENT_ID=
ENV CLIENT_SECRET=
ENV POST_AUTH_URI=
ENV GIT_PATH=$(which git)
EXPOSE 8000
CMD ["./devy-backend"]
