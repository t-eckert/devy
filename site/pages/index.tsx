import { PostMetadata } from "lib/post"
import Head from "next/head"

import Feed from "sections/Feed"
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

      <main className="px-2 max-w-3xl md:mt-12 mx-auto flex flex-col-reverse gap-12 md:flex-row md:gap-4">
        <div className="flex flex-col">
          <Feed />
        </div>
        <Intro />
      </main>
    </div>
  )
}

export default HomePage
