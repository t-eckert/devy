import { execSync } from "child_process"

execSync(
	"docker run --name devy-postgres -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:alpine"
)
