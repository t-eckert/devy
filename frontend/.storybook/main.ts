import type { StorybookConfig } from "@storybook/nextjs"
import path from "path"

const config: StorybookConfig = {
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-onboarding",
    "@storybook/addon-interactions",
    {
      name: "@storybook/addon-styling",
      options: {
        postCss: {
          implementation: require.resolve("postcss"),
        },
      },
    },
    "@storybook/addon-mdx-gfm",
    "@gfpacheco/storybook-tailwind-dark-mode",
  ],

  docs: {
    autodocs: false, // The background toggle is not working with autodocs. Just keep it off for now and figure it out later.
  },

  framework: {
    name: "@storybook/nextjs",
    options: {},
  },

  stories: [
    "../components/**/*.stories.@(js|jsx|mjs|ts|tsx)",
    "../app/**/*.stories.@(js|jsx|mjs|ts|tsx)",
    "../lockups/**/*.stories.@(js|jsx|mjs|ts|tsx)",
  ],

  webpackFinal: async (config) => {
    if (!config.resolve) {
      config.resolve = {}
    }

    config.resolve.alias = {
      ...config.resolve.alias,
      "@": path.resolve(__dirname, "../"),
    }

    return config
  },
}
export default config
