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

migrate-local-db:
	cd backend && cargo sqlx migrate run --database-url=postgres://postgres:postgres@localhost:5432

seed-local-db:
	docker cp $(shell pwd)/backend/seed.sql devy-postgres:/tmp/seed.sql
	docker exec -it devy-postgres psql -U postgres -f /tmp/seed.sql

