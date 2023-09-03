"use client"

import DataWindow from "./DataWindow"
import jwt from "jsonwebtoken"

import { useSearchParams } from "next/navigation"

export default function Jwt() {
	const searchParams = useSearchParams()

	const token = searchParams.get("token")

	if (!token) {
		return null
	}

	return <DataWindow name="JSON Web Token" data={jwt.decode(token)} />

}
