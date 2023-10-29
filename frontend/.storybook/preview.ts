import type { Preview } from "@storybook/react"
import { Inter } from "next/font/google"
import * as React from "react"

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
          value: "#18181B",
        },
        {
          name: "light",
          value: "#fafafa",
        },
      ],
    },
  },

  decorators: [
    (Story) =>
      React.createElement("div", {
        className: inter.className,
        children: React.createElement(Story),
      }),
  ],

  globalTypes: {
    darkMode: {
      defaultValue: true,
    },
  },
}

export default preview
