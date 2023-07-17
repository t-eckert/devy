export default interface Webhook {
	id?: string
	body: Object
}

export const webhookCreator = {
	new: async (webhook: Webhook): Promise<Webhook> => {
		console.error("NOT IMPLEMENTED")
		return webhook
	},
}
