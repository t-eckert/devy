import { PostMetadata } from "lib/post"
import Head from "next/head"
import Feed from "sections/Feed"

import Search from "sections/Search"

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

      <main>
        <div className="mt-24 mx-auto max-w-4xl flex flex-col">
          <Feed />
        </div>
      </main>
    </div>
  )
}

export default HomePage
