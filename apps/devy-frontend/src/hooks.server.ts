import type { Handle } from "@sveltejs/kit"
import { handleLogto } from '@logto/sveltekit';
import { LOGTO_APP_ID, LOGTO_ENDPOINT, LOGTO_APP_SECRET, LOGTO_ENCRYPTION_KEY } from "$env/static/private";

export const handle: Handle = handleLogto(
  {
    endpoint: LOGTO_ENDPOINT,
    appId: LOGTO_APP_ID,
    appSecret: LOGTO_APP_SECRET,
  },
  { encryptionKey: LOGTO_ENCRYPTION_KEY }
);
