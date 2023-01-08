export interface Props {
	status:
		| "requested"
		| "queued"
		| "cloning"
		| "parsing"
		| "uploading"
		| "complete"
}

const Upload: React.FC<Props> = ({ status }) => {
	if (status === "requested") {
		return <div className="rounded border px-2 py-1">Requested</div>
	}

	return <div>Upload</div>
}

export default Upload
