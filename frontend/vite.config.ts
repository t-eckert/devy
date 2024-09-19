import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import autoImport from "sveltekit-autoimport"

export default defineConfig({
  plugins: [
    autoImport({
      components: ["./src/lib/utils"]
    }),
    sveltekit()
  ],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
});
