# Global
version:
	@python3 ./tools/versioner.py

# Database
create-local-db:
	docker run \
		--name devy-postgres \
		-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
		-p 5432:5432 -d postgres:alpine 

destroy-local-db:
	docker stop devy-postgres
	docker rm devy-postgres

access-local-db:
	docker exec -it devy-postgres psql -U postgres

run-pgadmin:
	docker run -p 5050:80 \
		-e 'PGADMIN_DEFAULT_EMAIL=user@domain.com' \
		-e 'PGADMIN_DEFAULT_PASSWORD=password' \
		-d dpage/pgadmin4:latest

migrate-local-db:
	cd backend && cargo sqlx migrate run --database-url=postgres://postgres:postgres@localhost:5432

seed-local-db:
	docker cp $(shell pwd)/backend/seed.sql devy-postgres:/tmp/seed.sql
	docker exec -it devy-postgres psql -U postgres -f /tmp/seed.sql

