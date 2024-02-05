<script lang="ts">
	export let date: string

	const now = new Date()
	const then = new Date(date)
	const diff = now.getTime() - then.getTime()
	const seconds = Math.floor(diff / 1000)
	const minutes = Math.floor(seconds / 60)
	const hours = Math.floor(minutes / 60)
	const days = Math.floor(hours / 24)
	const weeks = Math.floor(days / 7)
	const months = Math.floor(weeks / 4)
	const years = Math.floor(months / 12)

	let relativeDate: string
	if (years > 0) {
		relativeDate = `${years} year${years > 1 ? "s" : ""} ago`
	} else if (months > 0) {
		relativeDate = `${months} month${months > 1 ? "s" : ""} ago`
	} else if (weeks > 0) {
		relativeDate = `${weeks} week${weeks > 1 ? "s" : ""} ago`
	} else if (days > 0) {
		relativeDate = `${days} day${days > 1 ? "s" : ""} ago`
	} else if (hours > 0) {
		relativeDate = `${hours} hour${hours > 1 ? "s" : ""} ago`
	} else if (minutes > 0) {
		relativeDate = `${minutes} minute${minutes > 1 ? "s" : ""} ago`
	} else {
		relativeDate = `${seconds} second${seconds > 1 ? "s" : ""} ago`
	}
</script>

<time datetime={date}>{relativeDate}</time>
