import { PostMetadata } from "interfaces"
import Head from "next/head"
import Feed from "sections/Feed"

import { Header } from "sections/Header"
import Search from "sections/Search"

type Props = {
  postsMetadata?: PostMetadata[]
}

const Home: React.FC<Props> = () => {
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
        <div className="mx-auto max-w-4xl flex flex-col">
          <Header />
          <div className="grid grid-cols-3 gap-4">
            <div className="col-span-2">
              <Feed />
            </div>
            <Search />
          </div>
        </div>
      </main>
    </div>
  )
}

export default Home
