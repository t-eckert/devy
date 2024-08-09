import { redirect } from "@sveltejs/kit"
import { PUBLIC_API } from "$env/static/public"

export function load() {
	return redirect(307, `${PUBLIC_API}/auth/sign-in`)
}
