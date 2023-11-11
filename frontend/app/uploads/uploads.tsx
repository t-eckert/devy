"use client"

import { useQuery } from "@tanstack/react-query"

import useSession from "@/lib/auth"
import useStore from "@/lib/useStore"
import api from "@/lib/api"
import { Upload } from "@/models"


import Json from "@/debug/json"

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

  return (
    <div className="flex flex-col gap-4 sm:gap-2">
      <Json data={uploads} />
    </div>
  )
}
