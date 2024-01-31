interface Data {
  changelog: string
}

export default async function data(): Promise<Data> {
  const res = await fetch(
    "https://raw.githubusercontent.com/t-eckert/devy/main/CHANGELOG.md",
    {
      next: {
        revalidate: 3600,
      },
    }
  )
  const changelog = await res.text()

  return {
    changelog,
  }
}
