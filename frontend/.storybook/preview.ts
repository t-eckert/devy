import type { Preview } from "@storybook/react"
import { themes } from "@storybook/theming"
import { Inter } from "next/font/google"
import * as React from "react"

import { withThemeByClassName } from "@storybook/addon-styling"

const inter = Inter({ subsets: ["latin"] })

import "../app/globals.css"

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    backgrounds: {
      default: "dark",
      values: [
        {
          name: "dark",
          value: "#09090b",
        },
        {
          name: "light",
          value: "#fafafa",
        },
      ],
    },
  },

  decorators: [
    withThemeByClassName({
      themes: {
        light: "light",
        dark: "dark",
      },
      defaultTheme: "light",
    }),
    (Story) =>
      React.createElement("div", {
        className: inter.className,
        children: React.createElement(Story),
      }),
  ],
}

export default preview
