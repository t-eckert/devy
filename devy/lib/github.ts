import Upload from "@/models/Upload"

interface Commit {}

interface Pusher {
	date?: string
	email: string | null
	name: string
	username?: string
}

interface GitHubUser {}

export interface PushWebhook {
	after: string
	base_ref: string | null
	before: string
	commits: Commit[]
	compare: string
	created: boolean
	deleted: boolean
	forced: boolean
	head_commit: boolean
	pusher: Pusher
	ref: string
	repository: string
	sender: GitHubUser
}

export const translatePushWebhookToUpload = (webhook: PushWebhook): Upload => {
	return {
		user: webhook.pusher.name,
	}
}
