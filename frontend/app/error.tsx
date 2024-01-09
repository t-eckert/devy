"use client"

import Error404 from "@/components/error/404"
import Error500 from "@/components/error/500"

export default function Error({
  error,
}: {
  error: Error & { digest?: string }
}) {
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
