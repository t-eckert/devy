# Global
version:
	@python3 ./tools/versioner.py

# Frontend
build-frontend:
	@cd frontend && npm run build

run-frontend:
	@cd frontend && npm run dev

# Backend
build-backend:
	@cd backend && cargo build --release

build-backend-container:
	@cd backend && docker build -t devy-backend .

run-backend:
	@cd backend && cargo watch -- cargo run

# Database
create-local-db:
	@docker run \
		--name devy-postgres \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d postgres:alpine 

destroy-local-db:
	@docker stop devy-postgres
	@docker rm devy-postgres

access-local-db:
	@docker exec -it devy-postgres psql -U postgres

run-pgadmin:
	@docker run -p 5050:80 \
		-e 'PGADMIN_DEFAULT_EMAIL=user@domain.com' \
		-e 'PGADMIN_DEFAULT_PASSWORD=password' \
		-d dpage/pgadmin4:latest

migrate-local-db:
	@cd backend && cargo sqlx migrate run --database-url=postgres://postgres:postgres@localhost:5432

seed-local-db:
	@docker cp $(shell pwd)/seed/ devy-postgres:/tmp/
	@docker exec -it devy-postgres psql -U postgres -f /tmp/seed/users.sql
	@docker exec -it devy-postgres psql -U postgres -f /tmp/seed/profiles.sql
	@docker exec -it devy-postgres psql -U postgres -f /tmp/seed/blogs.sql
	@docker exec -it devy-postgres psql -U postgres -f /tmp/seed/posts.sql
	@docker exec -it devy-postgres psql -U postgres -f /tmp/seed/likes.sql

# Integration Tests
run-integration-tests:
	@cd integration && \
		python3 -m venv .venv && \
		source .venv/bin/activate && \
		python3 -m pip install -r requirements.txt && \
		pytest
