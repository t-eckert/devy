export default interface Upload {
	id?: string
	previousUploadId?: string
	status?: string
	repo: string
	logs?: string[]
	createdAt?: string
	updatedAt?: string
}
