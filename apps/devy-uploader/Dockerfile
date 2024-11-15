FROM rust:1.82-slim-bullseye as builder
RUN mkdir /build
WORKDIR /build
COPY ./apps/devy-api ./apps/devy-api
COPY ./lib ./lib
COPY ./apps/devy-uploader ./apps/devy-uploader
COPY ./devyctl ./devyctl
COPY Cargo.toml Cargo.lock ./
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN SQLX_OFFLINE=true cargo build --bin devy-uploader --release


FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev git
WORKDIR /app
COPY --from=builder /build/target/release/devy-uploader .

ENV CALLBACK_URL=
ENV DATABASE_URL=
ENV ENCODING_PRIVATE_KEY=
ENV ENCODING_PUBLIC_KEY=
ENV GIT_PATH=/usr/bin/git
ENV GITHUB_APP_CLIENT_ID=
ENV GITHUB_APP_CLIENT_SECRET=
ENV GITHUB_APP_PRIVATE_KEY=
ENV REDIRECT_URL=

CMD ["./devy-uploader"]
