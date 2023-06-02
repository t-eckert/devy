export default function Home() {
  return (
    <>
      <main className="mx-auto mt-12 w-full max-w-6xl flex flex-row justify-between">
        <section className="flex flex-col items-start gap-2">
          <button className="px-2 py-0.5 border border-slate-700 shadow rounded-xl font-medium w-32 flex justify-start hover:pl-4 transition-all">
            Popular
          </button>
          <button className="px-2 py-0.5 rounded-xl w-32 flex justify-start hover:pl-4 transition-all">
            New
          </button>
        </section>
        <section className="w-full max-w-xl flex flex-col gap-2">
          <div>
            <h2 className="font-medium">Post title</h2>
            <p>Post description</p>
          </div>
          <div>
            <h2 className="font-medium">Post title</h2>
            <p>Post description</p>
          </div>
          <div>
            <h2 className="font-medium">Post title</h2>
            <p>Post description</p>
          </div>
          <div>
            <h2 className="font-medium">Post title</h2>
            <p>Post description</p>
          </div>
          <div>
            <h2 className="font-medium">Post title</h2>
            <p>Post description</p>
          </div>
        </section>
        <section>Secondary info</section>
      </main>
      <footer></footer>
    </>
  )
}
