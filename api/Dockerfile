FROM rust:1.71-slim-buster as builder
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN SQLX_OFFLINE=true cargo build --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/Rocket.toml .
EXPOSE 8080
CMD ["./backend"]
