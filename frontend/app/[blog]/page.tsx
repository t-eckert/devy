import Link from "@/components/link"

import { Blog, Post } from "@/models"
import api from "@/lib/api"

import BlogPosts from "./blog-posts"

interface Props {
  params: {
    blog: string
  }
}

export default async function BlogPage({ params }: Props) {
  const blog = await api.get<Blog>(`/v1/blogs/${params.blog}`, 600)
  const posts = await api.get<Post[]>(`/v1/blogs/${params.blog}/posts`, 600)

  return (
    <>
      <main className="mx-auto my-4 flex flex-col px-2 w-full max-w-6xl gap-12">
        {blog ? (
          <header>
            <h1 className="text-4xl font-semibold">{blog.name}</h1>
          </header>
        ) : (
          <header>Blog not found</header>
        )}
        <section className="flex flex-col gap-2">
          <h2 className="font-medium text-xl">Posts</h2>
          <BlogPosts posts={posts} />
        </section>
      </main>
    </>
  )
}
