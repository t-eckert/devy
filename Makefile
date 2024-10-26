# API
api-build:
	@SQLX_OFFLINE=true cargo build --bin ./backend/api --release

api-serve:
	@cd ./backend/api && RUST_LOG=DEBUG cargo watch -- cargo run

api-package:
	@md ./backend/ && podman build . -f ./api/Dockerfile -t devy-api


# Changelog
changelog-build:
	@podman build . -f ./changelog/Dockerfile -t devy-changelog

changelog-serve: changelog-build
	@podman run --rm \
	   --name devy-changelog -p 9000:5000 devy-changelog:latest


# DB
db-build:
	@podman build . -f ./backend/database/Dockerfile -t devy-test-db

db-serve: db-build
	@podman run --rm\
		--name devy-test-db \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d devy-test-db:latest

db-stop:
	@podman stop devy-test-db

db-access:
	@podman exec -it devy-test-db psql -U postgres


# Devyctl
devyctl-build:
	@cargo build --bin devyctl --release


# Frontend
frontend-build:
	@cd frontend && npm run build

frontend-serve:
	@cd frontend && npm run dev

frontend-package:
	@podman frontend . -f frontend/Dockerfile -t devy-frontend


# Tools
tools-setup:
	@cd tools && python3 -m venv .venv && . .venv/bin/activate && python3 -m pip install -r requirements.txt

venv:
	@cd tools && .venv/bin/activate

.PHONY: version
version:
	@python3 ./tools/versioner.py

changes:
	@python3 ./tools/changes.py

req:
	@python3 ./tools/req.py

test-integration: venv
	@python3 ./tools/test_integration.py
