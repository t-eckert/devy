import Link from "components/Link"
import { PostMetadata } from "lib/post"

export interface Props {
  postMetadata: PostMetadata
}

const Byline: React.FC<Props> = ({ postMetadata }) => {
  const { author, updatedAt, createdAt } = postMetadata

  return (
    <div className="w-full max-w-sm flex flex-col items-start gap-0.5 text-sm text-neutral-800">
      <div className="mt-0.5 mb-4 flex flex-row flex-wrap gap-1">
        {postMetadata.tags.map((tag, index) => (
          <Link
            href={`/search?tag=${tag}`}
            key={index}
            className={[
              "text-xs text-neutral-900 border rounded-full px-2 py-0.5 hover:-translate-y-1 hover:shadow-lg",
              index % 2 === 0 ? "hover:-rotate-2" : "hover:rotate-2",
            ].join(" ")}
          >
            {tag}
          </Link>
        ))}
      </div>
      <Link href={`/${author.username}`} className="font-medium">
        {author.name ?? author.username}
      </Link>
      {updatedAt !== null ? (
        <div>Published {new Date(createdAt).toLocaleDateString()}</div>
      ) : (
        <div>Updated {new Date(updatedAt).toLocaleDateString()}</div>
      )}
    </div>
  )
}

export default Byline
