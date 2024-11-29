import containerQueries from "@tailwindcss/container-queries"
import forms from "@tailwindcss/forms"
import typography from "@tailwindcss/typography"
import type { Config } from "tailwindcss"

export default {
  darkMode: "class",
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    fontFamily: {
      sans: ["Inter Variable", "sans-serif"],
      mono: ["IBM Plex Mono", "monospace"],
      serif: ["Playfair", "serif"]
    },
    extend: {
      fontFamily: {
        display: ["Playfair Display", "serif"]
      }
    }
  },

  plugins: [typography, forms, containerQueries]
} as Config
