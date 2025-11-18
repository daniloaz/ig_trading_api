<div align="center">
  <img src="assets/logo-ig.png" alt="IG Logo" width="200"/>
  
  <h1>IG Trading API</h1>
  <p><strong>A Rust client for the REST and Streaming APIs from IG.com</strong></p>

  [![Crates.io](https://img.shields.io/crates/v/ig_trading_api.svg)](https://crates.io/crates/ig_trading_api)
  [![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://github.com/daniloaz/ig-trading-api#readme)
  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/daniloaz/ig-trading-api/rust.yml?branch=main)](https://github.com/daniloaz/ig-trading-api/actions)
  [![GitHub stars](https://img.shields.io/github/stars/daniloaz/ig-trading-api.svg?style=social&label=Star)](https://github.com/daniloaz/ig-trading-api)
</div>

---

## 📋 Table of Contents

- [About](#about)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [REST API](#rest-api-example)
  - [Streaming API](#streaming-api-example)
- [API Coverage](#api-coverage)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

---

## 🎯 About

**IG Trading API** is a comprehensive Rust client library that provides seamless integration with [IG.com](https://www.ig.com/)'s REST and Streaming APIs. This library enables developers to build robust trading applications, automated trading strategies, and market analysis tools using the safety and performance benefits of Rust.

IG is a leading online trading platform offering CFDs, spread betting, and share dealing services across various markets including forex, indices, commodities, and cryptocurrencies.

### Why Choose This Library?

- **🦀 Type-Safe**: Leverages Rust's strong type system for compile-time API correctness
- **⚡ Async/Await**: Built on modern async Rust (Tokio) for high performance
- **🔒 Secure**: Implements secure authentication, session management, and proper secrets handling
- **📡 Real-Time**: Supports both REST and Lightstreamer-based streaming APIs
- **🛠️ Well-Tested**: Comprehensive integration tests ensure reliability
- **📝 Well-Documented**: Clear examples and documentation for all functionality
- **🔐 Security First**: Follows industry best practices with environment-based secrets management

---

## ✨ Features

### REST API Support
- **Account Management**: Retrieve account information, balances, and preferences
- **Session Management**: Secure authentication and session handling
- **Market Data**: Access to real-time and historical market data
- **Trading Operations**: Place, modify, and close positions
- **Order Management**: Create and manage working orders
- **Watchlists**: Create and manage custom watchlists
- **Price Alerts**: Set up and manage price alerts

### Streaming API Support
- **Real-Time Market Data**: Subscribe to live price updates
- **Account Updates**: Real-time account balance and position changes
- **Trade Confirmations**: Instant trade execution confirmations
- **Connection Management**: Automatic reconnection with exponential backoff
- **Signal Handling**: Graceful shutdown on SIGINT/SIGTERM

### Additional Features
- **Error Handling**: Comprehensive error handling with detailed error messages
- **Configuration Management**: YAML-based configuration with environment support
- **Colored Console Output**: Enhanced logging for better development experience
- **HTTP/HTTPS Support**: Secure communication with IG's servers
- **Demo Account Support**: Test your strategies on IG's demo environment

---

## 🔧 Prerequisites

Before using this library, ensure you have:

- **Rust 1.70.0 or higher** - [Install Rust](https://www.rust-lang.org/tools/install)
- **An IG Trading Account** - [Sign up at IG.com](https://www.ig.com/)
- **API Key** - Obtain from your IG account dashboard
- **OpenSSL** - Required for TLS connections
  - Ubuntu/Debian: `sudo apt-get install pkg-config libssl-dev`
  - Fedora: `sudo dnf install pkg-config openssl-devel`
  - macOS: `brew install openssl`

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ig_trading_api = "0.2.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Or use Cargo to add it directly:

```bash
cargo add ig_trading_api
cargo add tokio --features macros,rt-multi-thread
```

---

## ⚙️ Configuration

This project follows security best practices by separating sensitive credentials from application configuration:

- **`.env`** - Contains all sensitive data (API keys, passwords, account numbers, URLs)
- **`config.yaml`** - Contains only non-sensitive application behavior settings

### Step 1: Set Up Environment Variables

Copy the `.env.example` file to `.env`:

```bash
cp .env.example .env
```

Edit `.env` with your actual IG credentials:

```bash
# IG API Credentials
IG_API_KEY=your_actual_api_key_here
IG_USERNAME=your_username
IG_PASSWORD=your_password

# Account Numbers
IG_ACCOUNT_NUMBER_DEMO=XXXXX
IG_ACCOUNT_NUMBER_LIVE=XXXXX

# API URLs (default values, change only if needed)
IG_BASE_URL_DEMO=https://demo-api.ig.com/gateway/deal
IG_BASE_URL_LIVE=https://api.ig.com/gateway/deal

# Execution Environment
IG_EXECUTION_ENVIRONMENT=DEMO  # or LIVE for production
```

### Step 2: Configure Application Behavior (Optional)

The `config.yaml` file contains non-sensitive settings. You can modify these if needed:

```yaml
ig_trading_api:
  # Automatically log in when session expires
  auto_login: true
  
  # Logging mechanism (StdLogs or TracingLogs)
  logger: "StdLogs"
  
  # Session version for API requests
  session_version: 2
  
  # Max connection retry attempts for streaming
  streaming_api_max_connection_attempts: 3
```

### 🔒 Security Best Practices

- ✅ **NEVER commit your `.env` file** to version control (already in `.gitignore`)
- ✅ Keep sensitive credentials in `.env` only
- ✅ Use `.env.example` as a template (safe to commit)
- ✅ Use different credentials for DEMO and LIVE environments
- ✅ In production, consider using a secrets management service (AWS Secrets Manager, HashiCorp Vault, etc.)
- ✅ Rotate your API keys regularly

---

## 🚀 Usage

### REST API Example

```rust
use ig_trading_api::common::ApiConfig;
use ig_trading_api::rest_api::RestApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from .env and config.yaml
    // This automatically loads credentials from environment variables
    let config = ApiConfig::default();
    
    // Or explicitly load from environment and config
    let config = ApiConfig::from_env_and_config()?;
    
    // Create REST API client
    let api = RestApi::new(config).await?;
    
    // Get account information
    let (headers, accounts) = api.accounts_get().await?;
    println!("Accounts: {:#?}", accounts);
    
    // Get market data
    let (headers, market_nav) = api.market_navigation_get(None).await?;
    println!("Market Navigation: {:#?}", market_nav);
    
    // Search for markets
    let (headers, markets) = api.markets_search_get("EUR/USD".to_string()).await?;
    println!("Search Results: {:#?}", markets);
    
    Ok(())
}
```

### Streaming API Example

```rust
use ig_trading_api::common::ApiConfig;
use ig_trading_api::streaming_api::StreamingApi;
use ig_trading_api::rest_api::RestApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from .env and config.yaml
    let config = ApiConfig::default();
    
    // Create REST API client
    let rest_api = RestApi::new(config.clone()).await?;
    
    // Create streaming API client
    let mut streaming_api = StreamingApi::new(rest_api).await?;
    
    // Subscribe to market data
    streaming_api.subscribe_to_market("MARKET:CS.D.EURUSD.MINI.IP").await?;
    
    // Connect and start receiving updates
    streaming_api.connect().await;
    
    Ok(())
}
```

For more detailed examples, check the [`tests`](tests/) directory.

---

## 📊 API Coverage

### Implemented REST API Endpoints

| Category | Endpoints | Status |
|----------|-----------|--------|
| **Session** | Login, Logout, Refresh Token | ✅ |
| **Accounts** | Get Accounts, Preferences | ✅ |
| **Markets** | Navigation, Search, Details | ✅ |
| **Positions** | Get, Create, Update, Close | ✅ |
| **Orders** | Get, Create, Update, Delete | ✅ |
| **Watchlists** | Get, Create, Update, Delete | ✅ |
| **Prices** | Historical, Real-time | ✅ |

### Implemented Streaming API Features

| Feature | Status |
|---------|--------|
| Market Data Subscription | ✅ |
| Account Updates | ✅ |
| Trade Confirmations | ✅ |
| Automatic Reconnection | ✅ |
| Signal Handling | ✅ |

---

## 🧪 Testing

The project includes comprehensive integration tests for both REST and Streaming APIs.

### Run All Tests

```bash
cargo test
```

### Run Specific Tests

```bash
# REST API tests only
cargo test --test rest_api_integration_tests

# Streaming API tests only
cargo test --test streaming_api_integration_tests
```

### Run with Logging

```bash
RUST_LOG=debug cargo test -- --nocapture
```

**Note**: Integration tests require valid IG credentials in your `.env` file. Tests will run against your demo account by default. Make sure your `.env` file is properly configured before running tests.

---

## 📁 Project Structure

```
ig_trading_api/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Example executable
│   ├── common.rs           # Common types and utilities
│   ├── rest_api.rs         # REST API implementation
│   ├── rest_client.rs      # HTTP client wrapper
│   ├── rest_models.rs      # REST API data models
│   ├── rest_regex.rs       # Regex utilities
│   └── streaming_api.rs    # Streaming API implementation
├── tests/
│   ├── rest_api_integration_tests.rs
│   └── streaming_api_integration_tests.rs
├── assets/                 # Logo and images
├── .env.example           # Environment variables template (COPY TO .env)
├── .env                   # Your secrets (NOT in git)
├── config.yaml            # Non-sensitive app settings
├── config.default.yaml    # Default configuration template
├── Cargo.toml             # Project dependencies
├── LICENSE                # GPL-3.0 license
└── README.md              # This file
```

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

### Ways to Contribute

1. **Report Bugs**: Open an issue describing the bug with steps to reproduce
2. **Suggest Features**: Open an issue with your feature request
3. **Submit Pull Requests**: Fork the repo, create a feature branch, and submit a PR
4. **Improve Documentation**: Help us improve our documentation
5. **Write Tests**: Add more test coverage

### Development Setup

1. Fork and clone the repository:
```bash
git clone https://github.com/your-username/ig-trading-api.git
cd ig-trading-api
```

2. Create a feature branch:
```bash
git checkout -b feature/your-feature-name
```

3. Make your changes and ensure tests pass:
```bash
cargo test
cargo fmt
cargo clippy
```

4. Commit your changes:
```bash
git commit -m "Add: your feature description"
```

5. Push to your fork and submit a pull request

### Code Style

- Follow Rust's official style guide
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Write tests for new functionality
- Update documentation as needed

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0** - see the [LICENSE](LICENSE) file for details.

### What This Means

- ✅ You can use this software for any purpose
- ✅ You can modify the software to suit your needs
- ✅ You can distribute the software to your friends and neighbors
- ⚠️ If you distribute modified versions, you must also distribute the source code under GPL-3.0
- ⚠️ You must include the original copyright and license notices

For more information about GPL-3.0, visit: https://www.gnu.org/licenses/gpl-3.0.html

---

## 🙏 Acknowledgments

- **IG Group** - For providing the comprehensive trading API
- **Rust Community** - For the amazing ecosystem and tools
- **Lightstreamer** - For the real-time streaming technology
- All contributors who have helped improve this project

---

## 📞 Contact

**Daniel López Azaña**

- 🌐 Website: [www.daniloaz.com](https://www.daniloaz.com/en/)
- 📧 Email: daniloaz@gmail.com
- 💼 GitHub: [@daniloaz](https://github.com/daniloaz)

---

## ⚠️ Disclaimer

This software is provided "as is", without warranty of any kind. Trading financial instruments involves risk, and you should not trade with money you cannot afford to lose. This library is not affiliated with or endorsed by IG Group. Always test your code thoroughly on a demo account before using it in a live trading environment.

**USE AT YOUR OWN RISK**

---

<div align="center">
  <p>Made with ❤️ and Rust</p>
  <p>
    <a href="https://github.com/daniloaz/ig-trading-api/issues">Report Bug</a>
    ·
    <a href="https://github.com/daniloaz/ig-trading-api/issues">Request Feature</a>
  </p>
  
  <p>⭐ Star this repo if you find it useful!</p>
</div>
