import { redirect } from "@sveltejs/kit"
import { env } from "$env/dynamic/public"

export function load() {
	return redirect(307, `${env.PUBLIC_API}/auth/sign-in`)
}
