import { create } from "zustand"
import { persist } from "zustand/middleware"

export interface VersionStore {
	currentVersion: string
	lastVersion: string
	setCurrentVersion: (version: string) => void
	loadVersion: () => void
	isNewVersion: () => boolean
}

const useVersion = create<VersionStore>()(
	persist(
		(set) => ({
			currentVersion: "v0.4.1",
			lastVersion: "v0.3.6",
			setCurrentVersion: (version: string) => {
				set({ currentVersion: version })
			},
			loadVersion: () => {
				console.log("loadVersion")
			},
			isNewVersion: () => {
				return false
			},
		}),
		{ name: "version" }
	)
)

export default useVersion
