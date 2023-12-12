import { create } from "zustand"
import { persist } from "zustand/middleware"

import config from "@/config"

export interface VersionStore {
	currentVersion: string
	lastVersion: string
	setLastVersion: (version: string) => void
	loadVersion: () => void
	isNewVersion: () => boolean
}

const useVersion = create<VersionStore>()(
	persist(
		(set) => ({
			currentVersion: config.VERSION,
			lastVersion: "",
			setLastVersion: (version: string) => {
				set({ currentVersion: version, lastVersion: version })
			},
			loadVersion: () => {
				const version = localStorage.getItem("version")
				if (version) {
					set({ lastVersion: version })
				} else {
					set({ lastVersion: config.VERSION })
				}
			},
			isNewVersion: () => {
				return false
			},
		}),
		{ name: "version" }
	)
)

export default useVersion
