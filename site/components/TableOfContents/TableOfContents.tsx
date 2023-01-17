import { headers, Header } from "markdown"
import Link from "components/Link"

export interface Props {
  markdown: string
}

const formatHeader = (header: Header) => {
  const { id, level, value } = header

  switch (level) {
    case 1:
      return (
        <a
          href={`#${header.id}`}
          key={header.id}
          className="text-sm hover:underline"
        >
          {value}
        </a>
      )
    case 2:
      return (
        <a
          href={`#${id}`}
          key={id}
          className="pl-2 border-l text-xs hover:underline"
        >
          {value}
        </a>
      )
    case 3:
      return (
        <a
          href={`#${id}`}
          key={id}
          className="ml-2 pl-2 border-l text-xs hover:underline"
        >
          {value}
        </a>
      )
    case 4:
      return (
        <a
          href={`#${id}`}
          key={id}
          className="ml-4 pl-2 border-l text-xs hover:underline"
        >
          {value}
        </a>
      )
    case 5:
      return (
        <a
          href={`#${id}`}
          key={id}
          className="ml-6 pl-2 border-l text-xs hover:underline"
        >
          {value}
        </a>
      )
  }
}

const TableOfContents: React.FC<Props> = ({ markdown }) => {
  return (
    <div className="flex flex-col">{headers(markdown).map(formatHeader)}</div>
  )
}

export default TableOfContents
