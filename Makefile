version:
	@python3 ./tools/versioner.py

# Site
site-build:
	@cd site && npm run build

site-serve:
	@cd site && npm run dev

site-package:
	@cd images && docker build -t devy-site Dockerfile.site

storybook-serve:
	@cd site && npm run storybook

# API
api-build:
	@cd crates/backend && cargo build --release

api-serve:
	@cd crates/backend && RUST_LOG=DEBUG cargo watch -- cargo run

api-package:
	@cd images && docker build -t devy-api .

api-prepare-queries:
	@cd crates/backend && cargo sqlx prepare --database-url postgres://postgres:postgres@localhost:5432

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

