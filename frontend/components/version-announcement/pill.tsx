import { Cross2Icon } from "@radix-ui/react-icons"
import Link from "../link"

interface Props {
	version: string
	onClose?: () => void
}

export default function Pill({ version, onClose }: Props) {
	return (
		<div className={pill}>
			<span className="font-medium">
				New version{" "}
				<Link
					href="/changelog"
					className="decoration-blue-primary dark:decoration-red-high hover:text-blue-primary hover:dark:text-blue-high transition-all"
					onClick={onClose}
					variant={{ underline: false, styled: false }}
				>
					{version}
				</Link>
				.
			</span>
			<button
				className="aspect-square p-1 flex items-center justify-center rounded-full dark:text-blue-high hover:text-blue-primary transition-all"
				onClick={onClose}
			>
				<Cross2Icon />
			</button>
		</div>
	)
}

const pill = [
	"text-xs",
	"pl-3",
	"pr-1",
	"rounded-full",
	"flex",
	"flex-row",
	"items-center",
	"justify-center",
	"gap-2",

	"bg-blue-high",
	"opacity-90",
	"",

	"dark:bg-blue-primary",

	"shadow",
	"hover:shadow-md",
	"pointer-events-auto",
	"transition-all",
].join(" ")
