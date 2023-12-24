import { VariantProps, cva } from "cva"
import { ButtonHTMLAttributes } from "react"

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
	variant?: VariantProps<typeof styles>
}

export default function Button({ children, variant, ...props }: Props) {
	return (
		<button className={styles(variant)} {...props}>
			{children}
		</button>
	)
}

const styles = cva(
	[
		"px-2.5",
		"py-0.5",

		"pointer-cursor",
		"rounded-md",
		"select-none",

		"text-sm",
		"font-medium",

		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-neutral-2",
		"focus:ring-offset-1",
		"dark:focus:ring-neutral+2",

		"disabled:cursor-not-allowed",

		"transition-all",
	],
	{
		variants: {
			intent: {
				primary: [
					"border",
					"disabled:border-neutral+1 disabled:bg-neutral+1 disabled:text-neutral dark:disabled:border-neutral-1 dark:disabled:bg-neutral-1 dark:disabled:text-neutral",
				],
				secondary: [
					"disabled:border-neutral+1 disabled:bg-neutral+1 disabled:text-neutral dark:disabled:border-neutral-1 dark:disabled:bg-neutral-1 dark:disabled:text-neutral",
				],
				tertiary: [],
			},
			action: {
				none: [],
				create: [],
				modify: [],
				destroy: [],
			},
		},
		compoundVariants: [
			{
				intent: "primary",
				action: "none",
				className: [
					"text-neutral+3 border-blue-primary bg-blue-primary",
				].join(" "),
			},
			{
				intent: "primary",
				action: "create",
				className: [
					"bg-green-primary border-green-primary text-neutral+3",
				].join(" "),
			},
			{
				intent: "primary",
				action: "modify",
				className: [
					"bg-amber-primary border-amber-primary text-neutral+3",
				].join(" "),
			},
			{
				intent: "primary",
				action: "destroy",
				className: [
					"bg-red-primary border-red-primary text-neutral+3",
				].join(" "),
			},
			{
				intent: "secondary",
				action: "none",
				className: [
					"border text-blue-primary bg-blue-high border-blue-high ",
				].join(" "),
			},
			{
				intent: "secondary",
				action: "create",
				className: [
					"border text-green-primary bg-green-high border-green-high ",
				].join(" "),
			},
			{
				intent: "secondary",
				action: "modify",
				className: [
					"border text-amber-primary bg-amber-high border-amber-high",
				].join(" "),
			},
			{
				intent: "secondary",
				action: "destroy",
				className: [
					"border text-red-primary bg-red-high border-red-high",
				].join(" "),
			},
			{
				intent: "tertiary",
				action: "none",
				className: [
					"border-transparent text-neutral-3 border-transparent dark:text-neutral+3",
					"disabled:text-neutral disabled:dark:text-neutral",
				].join(" "),
			},
			{
				intent: "tertiary",
				action: "create",
				className: [
					"border-transparent text-green-primary border-transparent dark:text-green-primary dark:border-transparent",
					"disabled:text-neutral disabled:dark:text-neutral",
				].join(" "),
			},
			{
				intent: "tertiary",
				action: "modify",
				className: [
					"border-transparent text-amber-primary dark:text-amber-primary",
					"disabled:text-neutral disabled:dark:text-neutral",
				].join(" "),
			},
			{
				intent: "tertiary",
				action: "destroy",
				className: [
					"border-transparent text-red-primary dark:text-red-primary",
					"disabled:text-neutral disabled:dark:text-neutral",
				].join(" "),
			},
		],

		defaultVariants: {
			intent: "primary",
			action: "none",
		},
	}
)
