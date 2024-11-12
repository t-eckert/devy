export default interface TokenPayload<T> {
	sub: string
	body: T
	iss: string
	iat: number
	exp: number
	nbf: number
}
