import NewBlogForm from "@/components/new-blog-form"

export default async function NewBlogPage() {
  return (
    <section className="mx-auto my-4 px-2 w-full max-w-6xl flex flex-col gap-4">
      <h1 className="text-2xl font-semibold text-neutral-1 dark:text-neutral+1">
        Create a new blog
      </h1>

      <NewBlogForm />
    </section>
  )
}
