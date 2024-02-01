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
          "bg-neutral+3 dark:bg-neutral-2 text-neutral-2 dark:text-neutral+2",
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
