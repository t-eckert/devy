import type { Handle } from "@sveltejs/kit"
import { sequence } from "@sveltejs/kit/hooks"
import { handleLogto } from '@logto/sveltekit';
import { env } from '$env/dynamic/private';

const mine: Handle = async ({ event, resolve }) => {
  {
    const err = event.url.searchParams.get("error")
    if (err) console.log("error", err)

    const token = event.url.searchParams.get("token")
    if (token) {
      console.log("token", token)
      event.cookies.set("token", token, { path: "/", maxAge: 60 * 60 * 24 * 365 })
      event.url.searchParams.delete("token")
    }
  }

  const token = event.cookies.get("token")
  event.locals.token = token

  return await resolve(event)
}

const logto = handleLogto(
  {
    endpoint: env.LOGTO_ENDPOINT,
    appId: env.LOGTO_APP_ID,
    appSecret: env.LOGTO_APP_SECRET,
  },
  { encryptionKey: env.LOGTO_COOKIE_ENCRYPTION_KEY }
);

export const handle: Handle = sequence(mine, logto)
