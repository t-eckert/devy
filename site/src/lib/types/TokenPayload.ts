import Session from "./Session"

export default interface TokenPayload {
	sub: string
	value: Session
	iat: number
	exp: number
	nbf: number
}
