import { NextRequest, NextResponse } from "next/server"
import Uploader from "@/lib/Uploader"
import Upload from "@/models/Upload"

export async function POST(req: NextRequest) {
	const body = await req.json()
	if (!body) {
		return NextResponse.error()
	}

	const upload: Upload = {
		user: body.user,
	}

	const uploader = new Uploader(upload)

	const uploadId = await uploader.start()

	return NextResponse.json({ uploadId })
}

export async function GET(req: NextRequest) {
	console.dir(req)

	return NextResponse.json({ message: "Hello, World!" })
}
