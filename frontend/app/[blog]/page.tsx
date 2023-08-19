import blogs from "@/controllers/blog"
import DataWindow from "@/components/debugging/DataWindow"
import { Link } from "@/components/elements"
import Preview from "@/components/models/Post/Preview"
import Post from "@/models/Post"

interface Props {
  params: {
    blog: string
  }
}

export default async function Blog({ params }: Props) {
  const blog = await blogs.get.bySlug(params.blog)

  return (
    <>
      <DataWindow name="Blog data" data={blog} />
      <main className="mx-auto my-4 flex flex-col px-2 w-full max-w-6xl gap-12">
        <header>
          <Link
            href={`/profiles/${blog.profile.slug}`}
            variant={{ underline: true }}
          >
            {blog.profile.name}
          </Link>
          <h1 className="text-4xl font-semibold">{blog.name}</h1>
        </header>
        <section className="flex flex-col gap-2">
          <h2 className="font-medium text-xl">Posts</h2>
          {blog.posts.map((post: Post, i: number) => (
            <Preview key={i} {...post} />
          ))}
        </section>
      </main>
    </>
  )
}
