"use client"

import { useEffect, useState } from "react"

import useVersion from "@/lib/version"
import useStore from "@/lib/useStore"

import Pill from "./pill"

export default function VersionAnnouncement() {
	const version = useStore(useVersion, (version) => version)

	const [visible, setVisible] = useState<boolean>(false)

	useEffect(() => {
		if (version?.currentVersion === "") {
			setVisible(false)
		}
		if (version?.currentVersion !== version?.lastVersion) {
			setVisible(true)
		}
	}, [version?.currentVersion, version?.lastVersion])

	const onClose = () => {
		setVisible(false)
		version?.setLastVersion(version?.currentVersion || "")
	}

	return (
		<section
			className={[wrapper, visible ? "" : "-translate-y-10"].join(" ")}
		>
			<Pill version={version?.currentVersion || ""} onClose={onClose} />
		</section>
	)
}
const wrapper = [
	"hidden",
	"sm:flex",
	"fixed",

	"py-3",

	"z-50 ",

	"top-0",
	"left-0",
	"w-full",

	"flex-col",
	"items-center",
	"justify-center",
	"select-none",
	"pointer-events-none",
	"transition-all",
	"duration-300",
].join(" ")
