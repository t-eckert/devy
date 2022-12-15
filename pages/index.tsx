import { PostMetadata } from "interfaces"
import Head from "next/head"
import Feed from "sections/Feed"
import path from "path"
import { promises as fs } from "fs"

import { Header } from "sections/Header"

type Props = {
  postsMetadata?: PostMetadata[]
}

const Home: React.FC<Props> = ({ postsMetadata }) => {
  return (
    <div>
      <Head>
        <title>Divy</title>
        <meta
          name="description"
          content="Blog in Markdown from your GitHub repo."
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <div className="mx-auto max-w-xl flex flex-col">
          <Header />
          <Feed postsMetadata={postsMetadata} />
        </div>
      </main>
    </div>
  )
}

export async function getServerSideProps() {
  const dir = path.join(process.cwd(), "fixtures")
  const postsMetadata = await fs.readFile(dir + "/feed.json", "utf8")

  return {
    props: {
      postsMetadata: JSON.parse(postsMetadata),
    },
  }
}

export default Home
