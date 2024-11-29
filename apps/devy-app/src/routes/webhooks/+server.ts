import { json } from '@sveltejs/kit';

export async function POST() {

  console.log("Webhook received!")

  return json({ message: 'Webhook received!' });
}
