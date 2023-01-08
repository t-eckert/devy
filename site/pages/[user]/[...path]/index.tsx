import Article from "components/Article"
import { NextPageContext } from "next/types"
import { notFound } from "next/navigation"

import Post, { getPost } from "lib/post"
import parse from "markdown"
import Byline from "components/Byline"

interface Props {
  post: Post
  content: string
}

const PostPage: React.FC<Props> = ({ post, content }) => {
  return (
    <>
      <div className="z-10 absolute w-80 mt-60 ml-60">
        <Byline postMetadata={post} />
      </div>
      <div className="mx-auto my-6">
        <Article html={content} />
      </div>
    </>
  )
}

export const getServerSideProps = (context: NextPageContext) => {
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

  const post = getPost(user, path)

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
