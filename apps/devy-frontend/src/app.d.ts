// See https://svelte.dev/docs/kit/types#app.d.ts

import type { LogtoClient, UserInfoResponse } from '@logto/sveltekit';

// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      logtoClient: LogtoClient;
      user?: UserInfoResponse;
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export { };
