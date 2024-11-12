const wordsPerMinute = 200

export default function readingTime(text: string): number {
	return Math.ceil(text.split(" ").length / wordsPerMinute)
}
