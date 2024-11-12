import type { Validator } from "./Validator"

const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

const email: Validator = (value) => {
	return [emailRegex.test(value), "Invalid email"]
}

export default email
