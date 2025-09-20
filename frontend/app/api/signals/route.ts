export const runtime = "nodejs"
export const dynamic = "force-dynamic"

export async function GET() {
  const upstream = await fetch(
    `${process.env.NEXT_API_URL}/api/v${process.env.NEXT_API_VERSION}/signals`,
    {
      headers: { Accept: "text/event-stream" },
      cache: "no-store",
    },
  )

  // Пробрасываем поток как есть
  return new Response(upstream.body, {
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache, no-transform",
      Connection: "keep-alive",
      "X-Accel-Buffering": "no",
    },
  })
}
