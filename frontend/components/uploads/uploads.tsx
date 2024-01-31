"use client"

import Tabs from "@/components/elements/tabs"
import Card from "@/components/elements/card"

import useUploadsState from "./useUploadsState"

export default function Uploads() {
  const { uploads, selected, setSelected } = useUploadsState()

  return (
    <section className="w-full flex flex-col md:flex-row items-start gap-4">
      <Tabs
        labels={
          uploads?.map((upload) => ({
            id: upload?.id || "",
            name: upload?.createdAt || "",
          })) || []
        }
        selected={selected}
        setSelected={setSelected}
      />
      <div className="flex-1">
        <Card className="flex flex-col gap-2">
          <span>{uploads?.find((upload) => upload?.id === selected)?.id}</span>
          <span>
            {uploads?.find((upload) => upload?.id === selected)?.repo}
          </span>
          <span>
            {uploads
              ?.find((upload) => upload?.id === selected)
              ?.status.toUpperCase()}
          </span>
          <pre className="px-1 py-1 text-sm rounded w-full bg-neutral-lightest dark:bg-neutral-darkest">
            <code>
              {uploads
                ?.find((upload) => upload?.id === selected)
                ?.logs.join("\n")}
            </code>
          </pre>
        </Card>
      </div>
    </section>
  )
}
