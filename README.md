# CEX Arbitrage Scanner

A lightweight spot arbitrage **scanner** that compares prices across centralized exchanges (CEX) and highlights
opportunities after fees. It focuses on simple, explainable logic, dockerized services, and a small footprint for quick
local runs and demos.

> **Note:** This service scans markets and computes _net_ spreads using VIP0 spot fee assumptions. It **does not** place
> orders.

## Demo

<a href="https://cex-arb-scanner.dev.investerium.pro/" target="_blank" rel="noopener noreferrer">Live demo →</a>

## Features

- Cross-exchange spot price aggregation (public market data, no API keys required)
- Net-spread computation with **VIP0** maker/taker fees
- Dockerized **backend** and **frontend**
- Simple Makefile workflow for **dev** and **prod** profiles
- Extensible exchange adapters

## Supported Exchanges

Out of the box:

- **Binance**
- **Bybit**
- **OKX**
- **Bitget**
- **HTX**
- **Gate**

> You can extend/adjust the list by adding new adapters.

## Fee Model

- Uses per-exchange **VIP0** spot fees (taker) when computing net spreads

## Quickstart

### Prerequisites

- Docker & Docker Compose

### Configure environment

Copy an example env file and adjust values as needed:

```bash
cp .env.dev.example .env.dev

# or for production:
cp .env.prod.example .env.prod
```

### Start in development

http://localhost:30003

```bash
make dev build
make dev up -- -d

# ...

make dev down
```

### Start in production

http://localhost:30002

```bash
make prod build
make prod up -- -d

# ...

make prod down
```

### Notes on the Makefile

- Profiles are selected by adding a pseudo-target: dev or prod.
- Extra flags (like -d) are passed through to docker compose:
    - make dev up -d → docker compose -f compose.dev.yml up -d
    - make prod up -d → docker compose -f compose.prod.yml up -d

### Disclaimer

This repository is for research and educational purposes only. No financial advice. Use at your own risk.

### License

MIT