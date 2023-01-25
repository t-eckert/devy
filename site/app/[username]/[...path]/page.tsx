import { getPostByPath } from "lib/post"

interface Props {
  params: {
    username: string
    path: string[]
  }
}

const PostPage = async ({ params }: Props) => {
  const post = await getPostByPath(params.username, params.path.join("/"))

  return (
    <div className="mx-auto max-w-3xl my-6 px-2 md:px-0 flex flex-col">
      <pre>
        <code>{JSON.stringify(post, null, 2)}</code>
      </pre>
    </div>
  )
}

export default PostPage
