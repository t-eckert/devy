import { PostMetadata } from "lib/post"
import Head from "next/head"

import Feed from "sections/Feed"
import Search from "sections/Search"
import Intro from "sections/Intro"

type Props = {
  postsMetadata?: PostMetadata[]
}

const HomePage: React.FC<Props> = () => {
  return (
    <div>
      <Head>
        <title>Devy</title>
        <meta
          name="description"
          content="Blog in Markdown from your GitHub repo."
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="mt-12 mx-auto max-w-4xl grid grid-cols-3 gap-4">
        <div className="flex flex-col col-span-2">
          <Feed />
        </div>
        <div className="">
          <Intro />
        </div>
      </main>
    </div>
  )
}

export default HomePage
