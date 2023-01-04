import Post from "./Post"
import mocks from "components/Article/Article.mocks"

export const getPost = (authorname: string, title: string): Post => {
	return {
		title,
		author: {
			id: "7670827d-7b4f-4100-a4e3-d217079d5968",
			name: authorname,
		},
		path: `${authorname}/${title}`,
		tags: [],
		updated: "",
		likes: 20,
		markdown: "",
		html: mocks.base.html,
	}
}
