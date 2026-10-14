# 🛡️ Aegis-DLMM: Ultra-Low Latency Quantitative Engine for Solana

**Hackathon:** Colosseum Crypto World's Fair (Fall 2026)  
**Track:** Meteora Sidetrack (DLMM / DAMM v2)  
**Status:** Mainnet-Beta Ready  

Aegis is an institutional-grade, ultra-low latency high-frequency trading (HFT) and clearing engine built explicitly for the Solana ecosystem. By bridging a C++ AVX-512 vector math engine with a Rust/Axum networking layer via zero-copy shared memory, Aegis dynamically shifts liquidity across Meteora DLMM bins in micro-seconds to capture volatility spreads while completely neutralizing toxic flow and MEV sandwich attacks.

---

## ⚡ Technical Highlights

Unlike standard web3 applications that rely on heavy JSON-RPC polling and standard React state, Aegis brings TradFi HFT architecture to DeFi:

*   **Zero-Copy Memory Architecture:** Bypasses standard IPC serialization using `memmap2` and lock-free Single-Producer Single-Consumer (SPSC) ring buffers.
*   **Vectorized Quant Math:** Utilizes AVX-512 SIMD intrinsics in C++ to compute multi-dimensional **Hawkes Self-Exciting Point Processes** and **Avellaneda-Stoikov** inventory models concurrently, without branching.
*   **Anchor Bytemuck Deserialization:** Smart contracts use `bytemuck` to cast raw account data buffers directly to Rust structs in $O(1)$ time, saving tens of thousands of Compute Units (CUs) over standard `AccountDeserialize`.
*   **Mempool Bypass:** All dynamic bin rebalancing and liquidity shifting on Meteora is routed through atomic Jito bundles, guaranteeing execution order and preventing searcher front-running.
*   **Live Telemetry Dashboard:** React dashboard powered by Zustand, Recharts, and WebSockets visualizes order book depth, Meteora DLMM bin utilization, and pipeline latency.

---

## 🏗️ System Architecture (The 5 Phases)

### 1. Memory & FFI Layer (C++ / Rust)
The ingestion layer writes incoming market ticks directly to memory-mapped SPSC ring buffers. The C++ execution engine reads from the exact same physical memory layout, evaluating market toxicity using exponential decay kernels in sub-microsecond timeframes.

### 2. High-Frequency Telemetry API
A heavily optimized Axum WebSocket server streams binary payloads directly to frontend clients. Threads are manually pinned to isolated CPU cores using `libc::sched_setaffinity` to prevent Linux kernel context-switching latency.

### 3. On-Chain Execution (Solana & Meteora)
When the Hawkes intensity breaches toxicity thresholds, the engine builds an Anchor Cross-Program Invocation (CPI) targeting Meteora's `remove_liquidity` and `add_liquidity_by_strategy` instructions.

### 4. Autonomous Risk Engine
Continuously sweeps account collateral against mark prices. If maintenance margins are breached, the lock-free liquidation queue natively dispatches execution orders to square the position before the next block.

### 5. Live Telemetry Dashboard
A React and Vite interface provides a live DLMM bin distribution chart, market depth, and an engine latency gauge.

---

## 🚀 Getting Started

### Prerequisites
*   Linux environment (Ubuntu 24.04+ recommended for thread pinning and PMU profiling)
*   Rust toolchain (1.80+)
*   C++20 Compiler with AVX-512 support (`g++` or `clang++`)
*   Node.js 20+ & pnpm
*   Solana CLI (`v2.0+`)

### 1. Start the Telemetry Dashboard
The frontend is a Vite app. Its Vercel project root should be set to `frontend`.
```bash
cd frontend
npm install
npm run dev
```

The dashboard build can be checked with `npm run build`. Configure
`VITE_TELEMETRY_WS_URL` to the backend WebSocket endpoint (for example,
`ws://localhost:3000/ws/book` locally or `wss://your-api.example.com/ws/book`
in a deployed environment). Without an endpoint the dashboard builds normally
and shows the telemetry panels in their disconnected state.

### 2. Compile the Backend (Rust + C++ FFI)
Build the Rust/C++ backend with `cargo build --release` from the repository
root. The Cargo project is at the repo root, not in `src/`. Configure
`SOLANA_RPC_URL` and `SOLANA_WALLET_PATH` for the runtime.

The engine is a non-HTTP process. On Railway it must be deployed as its own
service, separate from the frontend dashboard, with root directory `/` and the
Dockerfile builder.

### 3. Run the Hardware Latency Profiler (Stress Test)
To verify cache-miss ratios and CPU core stability during the hackathon judging:

Bash
# Terminal 1: Run the load simulator (Blasts 5,000 TPS via WebSockets)
python3 scripts/load_simulator.py -c 20 -d 30 -r 0

# Terminal 2: Run the hardware profiler against the Rust binary
sudo ./scripts/latency_profiler.sh aegis-engine
🏆 Hackathon Context
This project was built for the Meteora Sidetrack at the Colosseum Crypto World's Fair (Fall 2026). It specifically demonstrates how institutional quant methodologies (Hawkes Processes) can be applied to Meteora's Dynamic Liquidity Market Maker (DLMM) architecture to actively defend LP positions against toxic flow.

Team: Elijah Branham Odhiambo

Track: Meteora DLMM / Infrastructure