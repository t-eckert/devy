/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./lib/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    colors: {
      "neutral-dark": "#09090B",
      "neutral-low": "#18181B",
      "neutral-medium": "#27272A",
      "neutral-high": "#71717A",
      "neutral-light": "#FAFAFA",
      "red-primary": "#DC2626",
      "red-high": "#FECACA",
      "amber-primary": "#CA8A04",
      "amber-high": "#FEF9C3",
      "green-primary": "#16A34A",
      "green-high": "#DCFCE7",
      "purple-primary": "#7C3AED",
      "purple-high": "#EDE9FE",
      "blue-primary": "#0284C7",
      "blue-high": "#E0F2FE",
    },
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
    },
  },
  plugins: [],
}
