import { exec } from "child_process"

import { NextResponse } from "next/server"

export async function GET(request: Request) {
	let out
	exec("which git", (error, stdout, stderr) => {
		if (error) {
			console.log(`error: ${error.message}`)
			out = `error: ${error.message}`
			return
		}
		if (stderr) {
			console.log(`stderr: ${stderr}`)
			out = `stderr: ${stderr}`
			return
		}
		console.log(`stdout: ${stdout}`)
		out = stdout
	})
	await new Promise((r) => setTimeout(r, 500))

	return NextResponse.json({ message: out })
}
