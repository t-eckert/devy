import Component404 from "./404"
import Component500 from "./500"

interface Props {
	error: Error
}

export default function ComponentError({ error }: Props) {
	if (error?.name === "NotFoundError") return <Component404 />
	if (error?.name === "ServerError") return <Component500 />

	return <div>Error</div>
}
