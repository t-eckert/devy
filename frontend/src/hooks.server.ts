import type { Handle } from "@sveltejs/kit"
import { sequence } from "@sveltejs/kit/hooks";
import { withClerkHandler } from "svelte-clerk/server"

export const handle: Handle = sequence(
  withClerkHandler(),
  ({ event, resolve }) => {

    // Forgive me JS gods.
    // @ts-expect-error to bypass serializability issues
    delete event.locals.auth.getToken
    // @ts-expect-error to bypass serializability issues
    delete event.locals.auth.has
    // @ts-expect-error to bypass serializability issues
    delete event.locals.auth.debug

    return resolve(event);
  }
);
