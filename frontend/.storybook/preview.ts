import type { Preview } from "@storybook/react"
import { themes } from "@storybook/theming"

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
  },
}

export const parameters = {
  darkMode: {
    dark: { ...themes.dark, appBg: "#09090b" },
  },
}

export default preview
