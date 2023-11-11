"use client"

import { useQuery } from "@tanstack/react-query"
import { useEffect, useState } from "react"

import Tabs from "@/components/tabs"

import useSession from "@/lib/auth"
import useStore from "@/lib/useStore"
import api from "@/lib/api"
import { Upload } from "@/models"
import Card from "@/components/card"

export default function Uploads() {
  const session = useStore(useSession, (session) => session)
  const username = session?.session?.user?.username

  const { data: uploads } = useQuery({
    queryKey: ["uploads", username],
    queryFn: () => {
      if (!username) return Promise.resolve([])
      return api.get<Upload[]>(`/v1/uploads/${username}`, 30)
    },
  })

  const [selected, setSelected] = useState<string>("")
  useEffect(() => {
    if (selected === "" && uploads) {
      setSelected(uploads[0]?.id || "")
    }
  }, [uploads, selected])

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
