/** @type {import('tailwindcss').Config} */
export default {
	darkMode: "class",
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		fontFamily: {
			sans: ["Inter", "sans-serif"],
			mono: ["Space Mono", "monospace"]
		}
	},
	plugins: [],
	safelist: ["dark", "w-screen", "h-screen", "border-stone-100", "h-64"]
}
