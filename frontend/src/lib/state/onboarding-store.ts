import { writable } from "svelte/store"

export interface OnboardingStore {
	isLoggedIn: boolean
	hasDevyInstalled: boolean
}

const onboardingStore = writable<OnboardingStore>({
	isLoggedIn: true,
	hasDevyInstalled: true
})

export default onboardingStore
