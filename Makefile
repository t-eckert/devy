.PHONY: version
version: 
	@python3 ./tools/versioner.py

# Site
site-build:
	@cd site && npm run build

site-serve:
	@cd site && npm run dev

site-package:
	@cd images && docker build -t devy-site images/Dockerfile.site

storybook-serve:
	@cd site && npm run storybook

# API
api-build:
	@cd crates && SQLX_OFFLINE=true cargo build --bin api --release

api-serve:
	@cd crates/api && RUST_LOG=DEBUG cargo watch -- cargo run

api-prepare-queries:
	@cd crates/db && cargo sqlx prepare --database-url postgres://postgres:postgres@localhost:5432

api-package:
	@docker build -t devy-api -f ./images/Dockerfile.api .

# DB
db-build:
	@cd images && docker build . -f Dockerfile.db.test -t devy-test-db

db-serve: db-build
	@docker run --rm\
		--name devy-test-db \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d devy-test-db:latest

db-access:
	@docker exec -it devy-test-db psql -U postgres

