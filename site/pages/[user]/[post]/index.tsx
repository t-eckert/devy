import Article from "components/Article"
import { NextPageContext } from "next/types"

import Post, { getPost } from "lib/post"

interface Props {
  post: Post
}

const PostPage: React.FC<Props> = ({ post }) => {
  return (
    <div className="mx-auto my-6">
      <Article html={post.html} />
    </div>
  )
}

export const getServerSideProps = (context: NextPageContext) => {
  let { user, post } = context.query

  // HACK
  user = user as string
  post = post as string

  const postData = getPost(user, post)

  return {
    props: {
      post: postData,
    },
  }
}

export default PostPage
