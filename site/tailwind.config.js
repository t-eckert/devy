/** @type {import('tailwindcss').Config} */
export default {
	darkMode: "class",
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			listStyleType: {
				current: "url('/chevron.svg')"
			}
		}
	},
	plugins: []
}
