FROM node:22-alpine AS base

WORKDIR /app/frontend

ENV NEXT_TELEMETRY_DISABLED=1



FROM base AS dev

COPY ./frontend/package*.json ./

RUN --mount=type=cache,target=/root/.npm npm ci

COPY ./frontend/. .



FROM base AS deps

COPY ./frontend/package*.json ./

RUN --mount=type=cache,target=/root/.npm npm ci



FROM base AS builder

COPY --from=deps /app/frontend/node_modules ./node_modules

COPY ./frontend/. .

RUN npm run build



FROM base AS runner

COPY --from=builder /app/frontend/public           ./public
COPY --from=builder /app/frontend/.next/standalone ./
COPY --from=builder /app/frontend/.next/static     ./.next/static

EXPOSE 3000

CMD ["node","server.js"]
