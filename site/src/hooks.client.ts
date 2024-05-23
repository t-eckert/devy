import { handleErrorWithSentry, replayIntegration } from "@sentry/sveltekit"
import * as Sentry from "@sentry/sveltekit"

Sentry.init({
	dsn: "https://f287c7e744498611eecc5ddb1f86a25b@o4506961359536128.ingest.us.sentry.io/4507087061450752",
	tracesSampleRate: 1.0,
	replaysSessionSampleRate: 0.1,
	replaysOnErrorSampleRate: 1.0,
	integrations: [replayIntegration()]
})

export const handleError = handleErrorWithSentry()
