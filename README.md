# Tensor Pyth Price Feed API

This repository provides a simple Rust-based Actix-web API for fetching real-time price feeds (such as ETH/USD, BTC/USD, EUR/USD) from the [Pyth Network](https://pyth.network/) on Solana.

## Features

- Fetches on-chain price data for supported assets using Pyth price feeds
- Exposes a REST API endpoint for querying prices by feed key
- Normalizes price output for easy consumption

## Usage

### Prerequisites

- Rust (edition 2021 or later)
- Solana toolchain
- [Pyth SDK for Solana](https://crates.io/crates/pyth-sdk-solana)
- [Actix-web](https://actix.rs/)

### Running the API

1. **Clone the repository:**
    ```bash
    git clone https://github.com/yourusername/tensor-pyth.git
    cd tensor-pyth
    ```

2. **Install dependencies:**
    ```bash
    cargo build
    ```

3. **Run the server:**
    ```bash
    cargo run
    ```

4. **Query a price feed:**
    ```
    GET http://127.0.0.1:6000/price/{feed_key}
    ```
    Replace `{feed_key}` with the Pyth price feed address (e.g., `3NBReDr1QqQ1uQe6r9h7tA4tWAPxF1i3r3dQyQz7F1oQ` for BTC/USD).

### Example Response

```json
{
  "price": 64321.12,
  "error": null
}
```

## Supported Price Feeds

You can find the latest Pyth price feed addresses [here](https://pyth.network/developers/price-feed-ids#solana-mainnet-beta).

Common examples:
- **BTC/USD:** `3NBReDr1QqQ1uQe6r9h7tA4tWAPxF1i3r3dQyQz7F1oQ`
- **ETH/USD:** `J83GAR6cB4bQfZ6gE2p7d7Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1`
- **EUR/USD:** `GZ1xQ1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1Q1`

## Project Structure

- `src/main.rs` — Main API server and route definitions
- `src/` — Additional modules and helpers

## License

MIT

## Acknowledgements

- [Pyth Network](https://pyth.network/)
- [Actix Web](https://actix.rs/)