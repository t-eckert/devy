/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./lib/**/*.{js,ts,jsx,tsx,mdx}",
    "./.storybook/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    colors: {
      "neutral-darkest": "#09090B",
      "neutral-darker": "#18181B",
      "neutral-medium": "#27272A",
      "neutral-lighter": "#f4f4f5",
      "neutral-lightest": "#FAFAFA",
      "neutral-3": "#09090B",
      "neutral-2": "#18181B",
      "neutral-1": "#27272A",
      neutral: "#71717A",
      "neutral+1": "#E4E4E7",
      "neutral+2": "#f4f4f5",
      "neutral+3": "#FAFAFA",
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
    keyframes: {
      slideDownAndFade: {
        from: { opacity: 0, transform: "translateY(-2px)" },
        to: { opacity: 1, transform: "translateY(0)" },
      },
      slideLeftAndFade: {
        from: { opacity: 0, transform: "translateX(2px)" },
        to: { opacity: 1, transform: "translateX(0)" },
      },
      slideUpAndFade: {
        from: { opacity: 0, transform: "translateY(2px)" },
        to: { opacity: 1, transform: "translateY(0)" },
      },
      slideRightAndFade: {
        from: { opacity: 0, transform: "translateX(-2px)" },
        to: { opacity: 1, transform: "translateX(0)" },
      },
    },
    animation: {
      slideDownAndFade: "slideDownAndFade 400ms cubic-bezier(0.16, 1, 0.3, 1)",
      slideLeftAndFade: "slideLeftAndFade 400ms cubic-bezier(0.16, 1, 0.3, 1)",
      slideUpAndFade: "slideUpAndFade 400ms cubic-bezier(0.16, 1, 0.3, 1)",
      slideRightAndFade:
        "slideRightAndFade 400ms cubic-bezier(0.16, 1, 0.3, 1)",
    },
  },
  plugins: [],
}
