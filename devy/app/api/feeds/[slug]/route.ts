import { NextRequest, NextResponse } from "next/server"
import { feedGetter } from "@/models/Feed"

export async function GET(
	_: NextRequest,
	{ params }: { params: { slug: string } }
) {
	if (!params.slug) return NextResponse.error()

	const feed = await feedGetter.bySlug(params.slug)

	return NextResponse.json(feed)
}
