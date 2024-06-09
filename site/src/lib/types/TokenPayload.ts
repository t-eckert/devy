import Session from "./Session"

export default interface TokenPayload {
	sub: string
	body: Session
	iss: string
	iat: number
	exp: number
	nbf: number
}
