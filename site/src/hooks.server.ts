import { NODE_ENV } from "$env/static/private"
import { sequence } from "@sveltejs/kit/hooks"
import * as Sentry from "@sentry/sveltekit"
import type { Handle } from "@sveltejs/kit"
import authHandle from "$lib/auth/hook"

if (NODE_ENV === "production") {
	Sentry.init({
		dsn: "https://f287c7e744498611eecc5ddb1f86a25b@o4506961359536128.ingest.us.sentry.io/4507087061450752",
		tracesSampleRate: 1
	})
}

export const handle: Handle = sequence(Sentry.sentryHandle(), async ({ event, resolve }) => {
	return authHandle({ event, resolve })
})

export const handleError = Sentry.handleErrorWithSentry()
