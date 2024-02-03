# Global
version:
	@python3 ./tools/versioner.py

# Frontend
build-frontend:
	@cd frontend && npm run build

serve-frontend:
	@cd frontend && npm run dev

serve-storybook:
	@cd frontend && npm run storybook

# Site
build-site:
	@cd site && npm run build

serve-site:
	@cd site && npm run dev

# Backend
build-backend:
	@cd backend && cargo build --release

build-backend-container:
	@cd backend && docker build -t devy-backend .

serve-backend:
	@cd backend && RUST_LOG=DEBUG cargo watch -- cargo run

backend-prepare-queries:
	@cd backend && cargo sqlx prepare --database-url postgres://postgres:postgres@localhost:5432

# Test database
build-test-db:
	@cd backend && docker build . -f test-db.Dockerfile -t devy-test-db

run-test-db: build-test-db
	@docker run --rm\
		--name devy-test-db \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d devy-test-db:latest

access-local-db:
	@docker exec -it devy-test-db psql -U postgres

run-pgadmin:
	@docker run -p 5050:80 \
		-e 'PGADMIN_DEFAULT_EMAIL=user@domain.com' \
		-e 'PGADMIN_DEFAULT_PASSWORD=password' \
		-d dpage/pgadmin4:latest

# Integration Tests
run-integration-tests:
	@cd integration && \
		python3 -m pip install -r requirements.txt && \
		python3 -m pytest -v
