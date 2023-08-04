import NextAuth from "next-auth"
import GithubProvider from "next-auth/providers/github"

import config from "@/config"

const handler = NextAuth({
	providers: [
		GithubProvider({
			clientId: config.AUTH.GITHUB.ID,
			clientSecret: config.AUTH.GITHUB.SECRET,
		}),
	],
})

export { handler as GET, handler as POST }
