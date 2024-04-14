/** @type {import('tailwindcss').Config} */
export default {
	darkMode: "class",
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		fontFamily: {
			sans: ["Inter", "sans-serif"],
			mono: ["Space Mono", "monospace"]
		},
		extend: {
			listStyleType: {
				current: "url('/chevron.svg')"
			}
		}
	},
	plugins: []
}
