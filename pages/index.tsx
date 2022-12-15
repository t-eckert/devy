import Head from "next/head";
import Image from "next/image";

export default function Home() {
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
        <h1>Divy</h1>
      </main>
    </div>
  );
}
