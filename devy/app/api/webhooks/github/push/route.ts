import { NextRequest, NextResponse } from "next/server"
import { webhookCreator } from "@/models/Webhook"
import { translatePushWebhookToUpload } from "@/lib/github"
import Uploader from "@/lib/Uploader"

export async function POST(req: NextRequest) {
	const body = await req.json()
	if (!body) {
		return NextResponse.error()
	}

	await webhookCreator.new({ body })

	const upload = translatePushWebhookToUpload(body)

	const uploader = new Uploader(upload)
	await uploader.start()

	return NextResponse.json({ message: "Webhook received" })
}
