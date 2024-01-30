import { useQuery } from "@tanstack/react-query"
import { useEffect, useState, SetStateAction, Dispatch } from "react"

import useSession from "@/lib/auth"
import useStore from "@/lib/useStore"
import api from "@/lib/api"
import { Upload } from "@/models"

interface UploadsState {
  uploads: Upload[]
  selected: string
  setSelected: Dispatch<SetStateAction<string>>
}

export default function useUploadsState(): UploadsState {
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

  return {
    uploads: uploads || [],
    selected,
    setSelected,
  }
}
