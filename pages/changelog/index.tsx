import Head from "next/head"

import parse from "markdown"

type Props = {
  changelog: string
}

const Changelog: React.FC<Props> = ({ changelog }) => {
  return (
    <div>
      <Head>
        <title>Divy: Changelog</title>
      </Head>

      <main>
        <div dangerouslySetInnerHTML={{ __html: changelog }} />
      </main>
    </div>
  )
}

export async function getStaticProps() {
  const resp = await fetch(
    "https://raw.githubusercontent.com/t-eckert/divy/main/CHANGELOG.md"
  )

  const changelog = parse(await resp.text())

  return {
    props: {
      changelog,
    },
  }
}

export default Changelog
