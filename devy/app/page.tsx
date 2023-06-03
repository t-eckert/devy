import Sidebar from "./Sidebar"

import { getTabs } from "@/models/Tab"

export default function Home() {
  const tabs = getTabs()

  return (
    <>
      <main className="mx-auto mt-12 w-full max-w-6xl flex flex-row justify-between">
        <Sidebar tabs={tabs} />
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
