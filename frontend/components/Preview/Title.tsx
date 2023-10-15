import { Link } from "../elements";

export default function Title({ title, url }: { title: string, url: string }) {
	return <Link href={url} className="font-medium text-zinc-50">{title}</Link>;
}
