FROM rust:1.71-slim-buster as builder
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN cargo build --release


FROM debian:buster-slim
COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/Rocket.toml .
EXPOSE 8080
CMD ["./backend"]
