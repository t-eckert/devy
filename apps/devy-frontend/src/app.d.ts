// See https://svelte.dev/docs/kit/types#app.d.ts

import type { Theme } from "$lib/state/theme.svelte";
import type { LogtoClient, UserInfoResponse } from "@logto/sveltekit";

// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      token?: string
      theme?: Theme
      logtoClient: LogtoClient
      user?: UserInfoResponse
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export { };
