FROM rust:1.76-slim-buster as builder
RUN mkdir /build
WORKDIR /build
COPY ./api ./api
COPY ./lib ./lib
COPY ./devyctl ./devyctl
COPY Cargo.toml Cargo.lock ./
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN SQLX_OFFLINE=true cargo build --bin api --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev git
WORKDIR /app
COPY --from=builder /build/target/release/api .

ENV CALLBACK_URL=
ENV DATABASE_URL=
ENV ENCODING_PRIVATE_KEY=
ENV ENCODING_PUBLIC_KEY=
ENV GIT_PATH=/usr/bin/git
ENV GITHUB_APP_CLIENT_ID=
ENV GITHUB_APP_CLIENT_SECRET=
ENV GITHUB_APP_PRIVATE_KEY=
ENV REDIRECT_URL=

EXPOSE 8000
CMD ["./api"]
