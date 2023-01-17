import { NextPageContext } from "next/types"
import { notFound } from "next/navigation"

import Post, { getPost } from "lib/post"
import parse from "markdown"
import Byline from "components/Byline"
import Article from "components/Article"
import TableOfContents from "components/TableOfContents"

interface Props {
  post: Post
  content: string
}

const PostPage: React.FC<Props> = ({ post, content }) => {
  return (
    <div className="mx-auto max-w-3xl my-6 px-2 md:px-0">
      <h1 className="mb-4 text-5xl font-semibold tracking-tighter leading-tight">
        {post.title}
      </h1>
      <div className="flex flex-col-reverse md:flex-row items-start gap-6">
        <Article html={content} />
        <div className="w-full flex flex-col gap-4 sticky top-2">
          <Byline postMetadata={post} />
          <div className="w-full hidden md:block">
            <TableOfContents markdown={post.markdown} />
          </div>
        </div>
      </div>
    </div>
  )
}

export const getServerSideProps = async (context: NextPageContext) => {
  let { user, path } = context.query

  if (!user || Array.isArray(user)) {
    notFound()
  }

  if (!path) {
    notFound()
  }

  if (Array.isArray(path)) {
    path = path.join("/")
  }

  const post = await getPost(user, path)

  if (!post) {
    notFound()
  }

  const content = parse(post.markdown)

  return {
    props: {
      post,
      content,
    },
  }
}

export default PostPage
