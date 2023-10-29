import { redirect } from "next/navigation"
import config from "@/config"

export async function GET() {
	redirect(`${config.API}/v1/auth/login`)
}
