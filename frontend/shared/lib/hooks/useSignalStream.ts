"use client"

import { useQuery, useQueryClient } from "@tanstack/react-query"
import { useCallback, useEffect, useRef, useState } from "react"

export interface TradingData {
  buy_exchange: string
  buy_price: string
  gross_percent: string
  net_percent: string
  sell_exchange: string
  sell_price: string
  symbol: string
  volume: string
  buy_link: string
  sell_link: string
}

export function useSignalStream(endpoint: string) {
  const queryClient = useQueryClient()
  const esRef = useRef<EventSource | null>(null)
  const reconnectTimer = useRef<number | null>(null)
  const [isConnected, setIsConnected] = useState(false)

  const attachHandlers = useCallback(
    (es: EventSource) => {
      es.onopen = () => setIsConnected(true)

      es.onmessage = (event) => {
        try {
          const newData: TradingData[] = JSON.parse(event.data)
          queryClient.setQueryData(["signal-data"], newData)
        } catch (e) {
          console.error("Error parsing SSE data:", e)
        }
      }

      es.onerror = (e) => {
        setIsConnected(false)

        es.close()
        if (esRef.current === es) esRef.current = null

        if (reconnectTimer.current == null) {
          reconnectTimer.current = window.setTimeout(() => {
            reconnectTimer.current = null
            open()
          }, 1000)
        }
      }
    },
    [queryClient],
  )

  const open = useCallback(() => {
    if (esRef.current) return

    const es = new EventSource(endpoint)

    esRef.current = es
    attachHandlers(es)
  }, [endpoint, attachHandlers])

  useEffect(() => {
    open()
    return () => {
      if (reconnectTimer.current != null) {
        clearTimeout(reconnectTimer.current)
        reconnectTimer.current = null
      }
      esRef.current?.close()
      esRef.current = null
      setIsConnected(false)
    }
  }, [open])

  const { data, isLoading, error } = useQuery<TradingData[]>({
    queryKey: ["signal-data"],
    queryFn: () => [],
    staleTime: Infinity,
    initialData: [],
  })

  return {
    data: data ?? [],
    isConnected,
    error,
  }
}
