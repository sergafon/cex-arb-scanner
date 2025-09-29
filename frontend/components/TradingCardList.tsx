"use client"

import { useSignalStream } from "@/shared/lib/hooks/useSignalStream"
import { TradingCard } from "@/components/TradingCard"
import { Wifi, WifiOff } from "lucide-react"
import { Separator } from "@/shared/ui/separator"

const countCex = 6

export function TradingCardList() {
  const { data, isConnected, error } = useSignalStream("/api/signals")

  if (error) {
    return (
      <div className="min-h-100 flex items-center justify-center">
        <div className="text-center space-y-4">
          <p className="text-error-foreground">{error.message}</p>
        </div>
      </div>
    )
  }

  return (
    <>
      <ConnectionStatus isConnected={isConnected} />

      <div className="flex justify-center gap-8 mb-8">
        <div className="text-center">
          {isConnected ? (
            <div className="text-2xl font-bold text-yellow-600 mb-2">
              {data.length}
            </div>
          ) : (
            <LoaderPairsNumber />
          )}
          <div className="text-sm text-muted-foreground">Active</div>
        </div>
        <Separator orientation="vertical" className="!h-12" />
        <div className="text-center">
          {isConnected ? (
            <div className="text-2xl font-bold text-green-600 mb-2">
              {data.filter((d) => Number.parseFloat(d.net_percent) > 0).length}
            </div>
          ) : (
            <LoaderPairsNumber />
          )}
          <div className="text-sm text-muted-foreground">Profitable</div>
        </div>
        <Separator orientation="vertical" className="!h-12" />
        <div className="text-center">
          {isConnected ? (
            <div className="text-2xl font-bold text-foreground mb-2">
              {countCex}
            </div>
          ) : (
            <LoaderPairsNumber />
          )}
          <div className="text-sm text-muted-foreground">CEX</div>
        </div>
      </div>

      {isConnected ? (
        data.length > 0 ? (
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {data.map((data, index) => (
              <TradingCard key={index} data={data} />
            ))}
          </div>
        ) : (
          <EmptyContent text="No routes right now — try again later" />
        )
      ) : (
        <EmptyContent text="Connecting to stream, waiting..." />
      )}
    </>
  )
}

const LoaderPairsNumber = () => {
  return (
    <div className="animate-pulse">
      <div className="h-8 w-8 bg-gray-100 rounded mx-auto mb-2"></div>
    </div>
  )
}

const ConnectionStatus = ({ isConnected }: { isConnected: boolean }) => {
  return (
    <div className="flex items-center justify-center gap-4 mb-8">
      {isConnected ? (
        <div className="flex items-center gap-2 text-green-600">
          <Wifi className="h-4 w-4" />
          <span className="text-sm">Connected</span>
        </div>
      ) : (
        <div className="flex items-center gap-2 text-red-600">
          <WifiOff className="h-4 w-4" />
          <span className="text-sm">Connecting...</span>
        </div>
      )}

      <div className="text-xs text-muted-foreground bg-muted/50 px-2 py-1 rounded-full">
        Updates every 5s
      </div>
    </div>
  )
}

const EmptyContent = ({ text }: { text: string }) => {
  return (
    <div className="min-h-100 flex items-center justify-center">
      <div className="text-center space-y-4">
        <p>{text}</p>
      </div>
    </div>
  )
}
