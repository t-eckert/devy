import Link from "components/Link"

const Intro: React.FC = () => {
  return <section className="flex flex-col items-start">
    <div className="flex flex-row gap-1.5 items-baseline">
      <h1 className="font-medium">Devy</h1>
      <div className="px-1.5 py-0.25 text-xs font-medium bg-yellow-300 text-yellow-900 rounded-full">ALPHA</div>
    </div>
    <p>
      Git push blog.
    </p>
    <p>
      Devy is under active development.
    </p>
    <ul className="text-sm">
      <li><Link href="/about" className="underline">About</Link></li>
      <li><Link href="/changelog" className="underline">Changelog</Link></li>
      <li><Link href="https://github.com/t-eckert/devy" className="underline">Repo</Link></li>
    </ul>
  </section>
}

export default Intro
