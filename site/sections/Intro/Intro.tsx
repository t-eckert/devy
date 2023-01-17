import Link from "components/Link"

const Intro: React.FC = () => {
  return (
    <section className="flex flex-col items-start">
      <p>Git push blog.</p>
      <p className="text-sm">Devy is under active development.</p>
      <ul className="text-sm flex flex-row gap-2 md:flex-col md:gap-0">
        <li>
          <Link href="/about" className="underline">
            About
          </Link>
        </li>
        <li>
          <Link href="/changelog" className="underline">
            Changelog
          </Link>
        </li>
        <li>
          <Link href="https://github.com/t-eckert/devy" className="underline">
            Repo
          </Link>
        </li>
      </ul>
    </section>
  )
}

export default Intro
