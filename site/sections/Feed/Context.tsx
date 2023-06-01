"use client"

import { useRef } from "react"

import { FeedState, useFeed } from "./useFeed"

interface Props extends FeedState {
	children: React.ReactNode
}

const Context: React.FC<Props> = ({ children, currentFeed, feeds }) => {
	const initialized = useRef(false)
	if (!initialized.current) {
		useFeed.setState({ currentFeed, feeds })
		initialized.current = true
	}
	return <>{children}</>
}

export default Context
