import Head from "next/head"
import { marked } from "marked"
import Article from "components/Article"

type Props = {
  changelog: string
}

const Changelog: React.FC<Props> = ({ changelog }) => {
  return (
    <div>
      <Head>
        <title>Devy: Changelog</title>
      </Head>

      <main>
        <Article html={changelog} />
      </main>
    </div>
  )
}

export async function getStaticProps() {
  const resp = await fetch(
    "https://raw.githubusercontent.com/t-eckert/devy/main/site/CHANGELOG.md"
  )

  const changelog = marked.parse(await resp.text())

  return {
    props: {
      changelog,
    },
  }
}

export default Changelog
