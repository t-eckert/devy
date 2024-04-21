import { sentrySvelteKit } from "@sentry/sveltekit"
import { sveltekit } from "@sveltejs/kit/vite"
import { optimizeDeps } from "vite"
import { defineConfig } from "vitest/config"

export default defineConfig({
	plugins: [
		sentrySvelteKit({
			sourceMapsUploadOptions: {
				org: "thomas-eckert",
				project: "devy-site"
			}
		}),
		sveltekit()
	],
	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"]
	},
	optimizeDeps: {
		exclude: ["@markdoc/markdoc"]
	}
})
