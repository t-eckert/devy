import subProcess from "child_process"

class Git {
	constructor() {
		subProcess.exec("which git", (error, stdout, stderr) => {
			if (error || stderr) {
				throw new Error(`git not found: ${error?.message || stderr}`)
			}
			console.log(`git found: ${stdout}`)
		})
	}

	clone(url: string, path: string) {
		subProcess.exec(`git clone ${url} ${path}`, (error, stdout, stderr) => {
			if (error || stderr) {
				throw new Error(`git clone failed: ${error?.message || stderr}`)
			}
			console.log(`git clone: ${stdout}`)
		})
	}
}

export default Git
