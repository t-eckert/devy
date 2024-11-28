// slug returns the given name in a slug format.
export default function slug(name: string): string {
	return name.replaceAll(" ", "-").toLowerCase().replaceAll("'", "")
}
