import { NextRequest, NextResponse } from "next/server"
import Upload, { uploadGetter } from "@/models/Upload"

export async function GET(
	_: NextRequest,
	{ params }: { params: { id: string } }
) {
	if (!params.id) return NextResponse.error()

	const upload = await uploadGetter.byId(params.id)

	return NextResponse.json(upload)
}
