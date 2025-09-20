import { TradingCardList } from "@/components/TradingCardList"

export default function Home() {
  return (
    <div className="bg-gradient-to-br from-background to-muted/20">
      <div className="container mx-auto px-4 py-8">
        <div className="mb-8 text-center">
          <p className="text-muted-foreground text-pretty text-xl max-w-3xl mx-auto leading-relaxed">
            Real-time analysis of arbitrage opportunities between cryptocurrency
            exchanges, calculated using VIP0 trading fees across all platforms.
          </p>
        </div>

        <TradingCardList />
      </div>
    </div>
  )
}
