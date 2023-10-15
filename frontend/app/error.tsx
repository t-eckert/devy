"use client"

import Error404 from "@/components/404"
import Error500 from "@/components/500"

export default function Error({
  error,
}: {
  error: Error & { digest?: string }
}) {
  // TODO: Log error
  // useEffect(() => {
  //   console.error(error)
  // }, [error])

  if (error.digest === "500") {
    return (
      <div className="py-20">
        <Error500 />
      </div>
    )
  }

  return (
    <div className="py-20">
      <Error404 />
    </div>
  )
}
