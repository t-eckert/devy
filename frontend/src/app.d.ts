import type { Security } from "svelte-clerk/env"

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      token?: string,
      security: Security
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export { }
