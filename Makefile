PARAMS := $(filter-out $@,$(MAKECMDGOALS))

# Devy Control
devyctl:
	@cd ./devyctl/ && RUST_LOG=DEBUG cargo run $(PARAMS)

devyctl-install:
	@cargo install --path ./devyctl

# Devy Frontend
frontend-build:
	@cd ./apps/devy-frontend && npm run build

frontend-serve:
	@cd ./apps/devy-frontend && npm run dev

frontend-package:
	@podman build . -f ./apps/devy-frontend/Dockerfile -t devy-frontend

frontend-test:
	@cd ./apps/devy-frontend && npm run test

frontend-deploy:
	@fly deploy --config ./infra/production/fly.frontend.toml


# Devy API
api-build:
	@SQLX_OFFLINE=true cargo build --bin ./apps/devy-api --release

api-serve:
	@cd ./apps/devy-api && RUST_LOG=DEBUG cargo watch -- cargo run

api-package:
	@md ./apps/devy-api && podman build . -f ./src/devy-api/Dockerfile -t devy-api

api-deploy:
	@fly deploy --config ./infra/production/fly.api.toml

# Devy Uploader
uploader-build:
	@cargo build --bin ./apps/devy-uploader --release

uploader-serve:
	@cd ./apps/devy-uploader && RUST_LOG=DEBUG cargo watch -- cargo run

uploader-package:
	@md ./apps/devy-uploader && podman build . -f ./apps/devy-uploader/Dockerfile -t devy-api


# Local Test Database
db-build:
	@podman build . -f ./infra/local/postgres/Dockerfile -t devy-test-db

db-serve: db-build
	@podman run --rm\
		--name devy-test-db \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d devy-test-db:latest

db-stop:
	@podman stop devy-test-db

db-access:
	@podman exec -it devy-test-db psql -U postgres

# Library
lib-build:
	@cargo build

lib-sqlx-prepare:
	@cd ./lib && cargo sqlx prepare --database-url postgres://postgres:postgres@localhost:5432
