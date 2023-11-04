"use client"

import { useState } from "react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"

interface Props {
	children: React.ReactNode
}

export default function Provider({ children }: Props) {
	const [queryClient] = useState(
		() =>
			new QueryClient({
				defaultOptions: {
					queries: {
						staleTime: 60 * 1000, // 1 minute
					},
				},
			}),
	)

	return (
		<div>
			<QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
			<ReactQueryDevtools initialIsOpen={false} />
		</div>
	)
}
