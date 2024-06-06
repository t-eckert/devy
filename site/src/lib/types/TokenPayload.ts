import Session from "./Session"

export default interface TokenPayload {
	sub: string
	body: Session
	iat: number
	exp: number
	nbf: number
}
