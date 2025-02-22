# Crypto Trading Engine 🚀

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A modular and extensible trading engine for cryptocurrency markets, built with Rust and Tokio.

## Features ✨

- 🧩 **Pluggable architecture** for easy customization
- ⚡ **Asynchronous task execution** powered by Tokio
- 🚦 **Event-driven design** with type-safe events
- 🔄 **Built-in scheduler** for task management
- 🛡️ **Error handling** with custom TradingEngineError type
- 📡 **Component-based architecture**:
  - **Connectors**: Data collection from exchanges/APIs
  - **Aggregator**: Data normalization and consolidation
  - **Strategies**: Market analysis and signal generation
  - **Executors**: Trade execution and order management

## Installation 📦

Add this to your `Cargo.toml`:

```toml
[dependencies]
crypto-trading-engine = "0.1.0"
tokio = { version = "1.43.0", features = ["full"] }
async-trait = "0.1.85"
thiserror = "2.0.11"
```

## Quick Start 🚀

```rust
use crypto_trading_engine::{TradingEngineBuilder, TradingEngineError};
use async_trait::async_trait;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum TradingEvent {
    MarketDataUpdate,
    TradeSignal,
    Shutdown,
}

// Implement your components
struct MyConnector;
struct MyAggregator;
struct MyStrategy;
struct MyExecutor;

#[async_trait]
impl Connector for MyConnector { /* ... */ }
#[async_trait]
impl Aggregator for MyAggregator { /* ... */ }
#[async_trait]
impl Strategy<TradingEvent> for MyStrategy { /* ... */ }
#[async_trait]
impl Executor<TradingEvent> for MyExecutor { /* ... */ }

#[tokio::main]
async fn main() -> Result<(), TradingEngineError> {
    // Create event bus with your custom events
    let event_bus = EventBus::new(vec![
        (TradingEvent::MarketDataUpdate, ChannelConfig {
            capacity: 1000,
            description: "Market data channel".to_string(),
        }),
        (TradingEvent::TradeSignal, ChannelConfig {
            capacity: 100,
            description: "Trade signals channel".to_string(),
        }),
        (TradingEvent::Shutdown, ChannelConfig {
            capacity: 10,
            description: "Shutdown channel".to_string(),
        }),
    ]);

    // Build and start the engine
    let mut engine = TradingEngineBuilder::new(event_bus)
        .with_connector(Box::new(MyConnector))
        .with_aggregator(Box::new(MyAggregator))
        .with_strategy(Box::new(MyStrategy))
        .with_executor(Box::new(MyExecutor))
        .build()?;

    engine.start().await?;
    
    // Run until shutdown
    tokio::signal::ctrl_c().await?;
    engine.shutdown().await?;
    
    Ok(())
}
```

## Architecture 🏗️

The engine follows a modular architecture with clearly defined components:

1. **Connectors** (N): 
   - Connect to external data sources (exchanges, APIs, etc.)
   - Retrieve raw market data
   - Implement the `Connector` trait

2. **Aggregator** (1):
   - Normalizes data from multiple connectors
   - Aggregates data into consistent format
   - Implements the `Aggregator` trait

3. **Strategies** (N):
   - Analyze market data
   - Calculate indicators
   - Detect trading opportunities
   - Implement the `Strategy` trait

4. **Executors** (N):
   - Execute trades via API calls
   - Manage order placement
   - Implement the `Executor` trait

## Data Flow 📊

```
Connector(N) -> Aggregator(1) -> Strategy(N) -> Executor(N)
```

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
