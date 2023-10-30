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
        date: /Date$/,
      },
    },
    layout: "centered",
  },

  decorators: [
    (Story) =>
      React.createElement("div", {
        className: [
          inter.className,
          "text-neutral-low dark:text-neutral-light",
        ].join(" "),
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
