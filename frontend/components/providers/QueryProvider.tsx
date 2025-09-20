"use client"

import { QueryClientProvider } from "@tanstack/react-query"
import { PropsWithChildren, Suspense } from "react"
import { getQueryClient } from "@/shared/lib/api/getQueryClient"

export const QueryProvider = ({ children }: PropsWithChildren) => {
  return (
    <Suspense fallback={null}>
      <QueryClientProvider client={getQueryClient()}>
        {children}
      </QueryClientProvider>
    </Suspense>
  )
}
