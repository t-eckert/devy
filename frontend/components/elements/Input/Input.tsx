import { InputHTMLAttributes } from "react"

interface Props extends InputHTMLAttributes<HTMLInputElement> {}

export default function Input({ name }: Props) {
	return <input name={name}></input>
}
