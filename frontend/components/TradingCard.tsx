import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card"
import { Badge } from "@/shared/ui/badge"
import { Separator } from "@/shared/ui/separator"
import { ArrowRightLeft, TrendingDown, TrendingUp } from "lucide-react"
import { TradingData } from "@/shared/lib/hooks/useSignalStream"

interface TradingCardProps {
  data: TradingData
}

function formatToMaxDecimals(...nums: string[]): string[] {
  const maxDecimals = Math.max(
    ...nums.map((s) => {
      return s.includes(".") ? s.split(".")[1].replace(/0+$/, "").length : 0
    }),
  )

  return nums.map((s) => {
    const n = Number(s)

    return n.toLocaleString(undefined, {
      minimumFractionDigits: maxDecimals,
      maximumFractionDigits: maxDecimals,
    })
  })
}

function formatDecimals(num: number): string {
  const isInteger = Number.isInteger(num)

  return num.toLocaleString(undefined, {
    minimumFractionDigits: 0,
    maximumFractionDigits: isInteger ? 0 : 4,
  })
}

export function TradingCard({ data }: TradingCardProps) {
  const grossPercent = Number.parseFloat(data.gross_percent)
  const netPercent = Number.parseFloat(data.net_percent)
  const volume = formatDecimals(Number(data.volume))
  const isProfit = netPercent > 0
  const symbol = data.symbol.toUpperCase()
  const [buy_price, sell_price] = formatToMaxDecimals(
    data.buy_price,
    data.sell_price,
  )

  return (
    <Card className="hover:shadow-lg transition-all">
      <CardHeader className="border-b">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <CardTitle className="text-2xl font-bold tracking-tight">
              {symbol}
            </CardTitle>
            {isProfit ? (
              <TrendingUp className="h-5 w-5 text-green-600" />
            ) : (
              <TrendingDown className="h-5 w-5 text-red-600" />
            )}
          </div>
          <Badge
            variant={isProfit ? "positive" : "destructive"}
            className="text-sm font-bold px-3 py-1"
          >
            {isProfit ? "+" : ""}
            {netPercent.toFixed(3)}%
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="pt-6">
        <div className="grid grid-cols-3 gap-6 mb-6">
          <div className="text-center space-y-2">
            <div className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
              Buy
            </div>
            <a
              href={data.buy_link}
              target="_blank"
              rel="noopener noreferrer"
              className="block"
            >
              <Badge
                variant="outline"
                className="w-full justify-center font-semibold text-blue-600 border-blue-200 hover:bg-blue-50 transition-colors cursor-pointer"
              >
                {data.buy_exchange}
              </Badge>
            </a>
            <div className="text-xl font-mono font-bold text-foreground">
              ${buy_price}
            </div>
          </div>

          <div className="flex items-center justify-center">
            <div className="p-2 rounded-full bg-muted">
              <ArrowRightLeft className="h-4 w-4 text-muted-foreground" />
            </div>
          </div>

          <div className="text-center space-y-2">
            <div className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
              Sell
            </div>
            <a
              href={data.sell_link}
              target="_blank"
              rel="noopener noreferrer"
              className="block"
            >
              <Badge
                variant="outline"
                className="w-full justify-center font-semibold text-green-600 border-green-200 hover:bg-green-50 transition-colors cursor-pointer"
              >
                {data.sell_exchange}
              </Badge>
            </a>
            <div className="text-xl font-mono font-bold text-foreground">
              ${sell_price}
            </div>
          </div>
        </div>

        <Separator className="my-4" />

        <div className="space-y-3">
          <div className="flex justify-between items-center p-3 rounded-lg bg-muted/50">
            <span className="text-sm font-medium text-muted-foreground">
              Volume
            </span>
            <span className="font-mono font-bold text-foreground">
              {volume}
            </span>
          </div>

          <div className="flex justify-between items-center p-3 rounded-lg bg-muted/50">
            <span className="text-sm font-medium text-muted-foreground">
              Gross
            </span>
            <Badge
              variant={grossPercent > 0 ? "secondary" : "outline"}
              className="font-mono font-semibold"
            >
              {grossPercent > 0 ? "+" : ""}
              {grossPercent.toFixed(3)}%
            </Badge>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
