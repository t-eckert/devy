import Collection from "@/components/post/collection"

import data from "./data"

interface Props {
  params: {
    blog: string
  }
}

export default async function BlogPage({ params }: Props) {
  const { blog, posts } = await data(params.blog)

  return (
    <>
      <main className="mx-auto my-4 flex flex-col px-2 w-full max-w-6xl gap-12">
        <header>
          <h1 className="text-4xl font-semibold">{blog.name}</h1>
        </header>
        <section className="flex flex-col gap-2">
          <h2 className="font-medium text-xl">Posts</h2>
          <Collection posts={posts} />
        </section>
      </main>
    </>
  )
}
