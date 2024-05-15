.PHONY: version
version:
	@python3 ./tools/versioner.py

changes:
	@python3 ./tools/changes.py

test-integration:
	@cd integration && \
		python3 -m pip install -r requirements.txt && \
		python3 run.py

# Site
site-build:
	@cd site && npm run build

site-serve:
	@cd site && npm run dev

site-package:
	@docker build . -f images/Dockerfile.site -t devy-site

storybook-serve:
	@cd site && npm run storybook

# API
api-build:
	@SQLX_OFFLINE=true cargo build --bin api --release

api-serve:
	@cd api && RUST_LOG=DEBUG cargo watch -- cargo run

api-prepare-queries:
	@cd db && cargo sqlx prepare --database-url postgres://postgres:postgres@localhost:5432

api-package:
	@docker build . -f ./images/Dockerfile.api -t devy-api

# DB
db-build:
<<<<<<< HEAD
	@docker build . -f ./integration/db.Dockerfile -t devy-test-db
=======
<<<<<<< HEAD
	@docker build . -f ./integration/db.Dockerfile -t devy-test-db
=======
	@docker build . -f ./lib/Dockerfile -t devy-test-db
>>>>>>> 48ff75a (lib:refactor into single crate)
>>>>>>> @{-1}

db-serve: db-build
	@docker run --rm\
		--name devy-test-db \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d devy-test-db:latest

db-stop:
	@docker stop devy-test-db

db-access:
	@docker exec -it devy-test-db psql -U postgres

db-prepare:
	@cd db && cargo sqlx prepare --database-url postgres://postgres:postgres@localhost:5432
