# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-11-18

### 🔒 Security - BREAKING CHANGES

This release introduces critical security improvements by separating sensitive credentials from application configuration. **This is a breaking change** that requires migration steps.

#### ❌ Previous Implementation (Insecure)
All configuration including secrets was stored in `config.yaml`:
- API keys, passwords, and credentials in version control
- High risk of accidentally exposing sensitive data
- Poor separation of concerns
- Difficult to manage different environments
- Non-compliant with security standards

#### ✅ New Implementation (Secure)
- **Secrets in `.env` file** - All sensitive data isolated in environment variables
- **Automatic `.gitignore`** - `.env` file is never committed to version control
- **Clear separation** - Application settings in `config.yaml`, secrets in `.env`
- **Environment-specific configs** - Easy to manage multiple environments
- **Industry standard** - Follows 12-factor app and OWASP best practices

### ✨ Added

- **Environment Variable Support**: New loading system via `dotenvy` crate
- `ApiConfig::from_env_and_config()` - Explicitly load from environment and config
- `ApiConfig::load_env()` - Helper for loading `.env` file
- `.env.example` - Template file for environment variables
- `MIGRATION_GUIDE.md` - Complete migration guide from v0.2.3 to v0.3.0
- `SECURITY_IMPROVEMENTS.md` - Detailed security enhancements documentation
- Professional README.md with:
  - IG logo and branding
  - shields.io badges (Crates.io, License, Rust, Build Status, Stars)
  - Comprehensive documentation following open-source best practices
  - Security-focused configuration guide
  - Table of contents and professional structure

### ✅ Testing

All integration tests pass successfully:
- **14/14 REST API tests**: 100% PASS
- **Compilation**: Debug and Release builds successful
- **Configuration loading**: Verified working correctly
- **Authentication**: All authentication flows working

### 🔧 Changed - BREAKING

- **`ApiConfig::default()`** now loads from environment variables + config.yaml
- **`config.yaml`** now only contains non-sensitive application settings:
  - `auto_login`
  - `logger`
  - `session_version`
  - `streaming_api_max_connection_attempts`
- **Sensitive fields** in `ApiConfig` now use `#[serde(skip_deserializing)]`:
  - `api_key` (from `IG_API_KEY`)
  - `username` (from `IG_USERNAME`)
  - `password` (from `IG_PASSWORD`)
  - `account_number_demo` (from `IG_ACCOUNT_NUMBER_DEMO`)
  - `account_number_live` (from `IG_ACCOUNT_NUMBER_LIVE`)
  - `account_number_test` (from `IG_ACCOUNT_NUMBER_TEST`)
  - `base_url_demo` (from `IG_BASE_URL_DEMO`)
  - `base_url_live` (from `IG_BASE_URL_LIVE`)
  - `execution_environment` (from `IG_EXECUTION_ENVIRONMENT`)

### 📝 Documentation

- Enhanced README.md with professional structure and badges
- Added IG logo to project documentation
- Created comprehensive migration guide ([MIGRATION_GUIDE.md](MIGRATION_GUIDE.md))
- Added security best practices section
- Updated all code examples to use new configuration method
- Created [SECURITY_IMPROVEMENTS.md](SECURITY_IMPROVEMENTS.md) with detailed analysis

### 🏗️ Internal

- Added `Default` trait implementation for `ExecutionEnvironment`
- Improved error messages for missing environment variables
- Added `dotenvy = "0"` dependency to Cargo.toml
- Updated `.gitignore` to include `.env`

### 🔐 Security Benefits

| Aspect | Before (v0.2.3) | After (v0.3.0) |
|--------|-----------------|----------------|
| **Secrets in Git** | ❌ Yes (risky) | ✅ No (protected) |
| **Accidental Exposure** | ❌ High risk | ✅ Protected by .gitignore |
| **Environment Management** | ❌ Difficult | ✅ Easy with .env files |
| **Compliance** | ❌ Non-compliant | ✅ OWASP, 12-factor compliant |
| **Secret Rotation** | ❌ Hard | ✅ Easy |
| **Audit Trail** | ❌ Poor | ✅ Good |

### 📊 Compliance Standards

This release now complies with:
- ✅ **OWASP Top 10** - Addresses A02:2021 – Cryptographic Failures
- ✅ **12-Factor App** - Factor III: Config
- ✅ **CIS Controls** - Control 3: Data Protection
- ✅ **NIST Guidelines** - Configuration Management
- ✅ **PCI DSS** - Requirement 8.2 (if applicable)

### 🔄 Migration Required

To upgrade from v0.2.3 to v0.3.0:

1. **Copy the template:**
   ```bash
   cp .env.example .env
   ```

2. **Move your credentials from `config.yaml` to `.env`:**
   ```bash
   # In .env file:
   IG_API_KEY=your_api_key
   IG_USERNAME=your_username
   IG_PASSWORD=your_password
   IG_ACCOUNT_NUMBER_DEMO=XXXXX
   IG_ACCOUNT_NUMBER_LIVE=XXXXX
   IG_BASE_URL_DEMO=https://demo-api.ig.com/gateway/deal
   IG_BASE_URL_LIVE=https://api.ig.com/gateway/deal
   IG_EXECUTION_ENVIRONMENT=DEMO
   ```

3. **Update your `config.yaml`** to only include non-sensitive settings:
   ```yaml
   ig_trading_api:
     auto_login: true
     logger: "StdLogs"
     session_version: 2
     streaming_api_max_connection_attempts: 3
   ```

4. **Update your code** (optional, default still works):
   ```rust
   // Old way (still works)
   let config = ApiConfig::default();
   
   // New explicit way (recommended)
   let config = ApiConfig::from_env_and_config()?;
   ```

**For detailed information, see [RELEASE_NOTES_v0.3.0.md](RELEASE_NOTES_v0.3.0.md)**

### ⚠️ Important Security Notes

1. **Never commit `.env`** - Already in `.gitignore`
2. **Use `.env.example`** - As a template for new developers
3. **Rotate credentials regularly** - Change API keys periodically
4. **Use secrets managers in production** - Consider AWS Secrets Manager, Vault, etc.
5. **Different credentials per environment** - Never reuse keys between DEMO and LIVE

---

## [0.2.3] - Previous Release

### Features
- Full REST API implementation
- Streaming API support via Lightstreamer
- Account management
- Market data access
- Trading operations (positions, orders)
- Order management (working orders)
- Watchlist functionality
- Historical data access
- Session management

### API Coverage
- Session endpoints (login, logout, refresh)
- Account endpoints (get accounts, preferences)
- Market endpoints (navigation, search, details)
- Position endpoints (create, read, update, close)
- Order endpoints (create, read, update, delete)
- Watchlist endpoints (CRUD operations)
- Historical data endpoints (prices, transactions, activity)

---

## Links

- **Release Notes**: [RELEASE_NOTES_v0.3.0.md](RELEASE_NOTES_v0.3.0.md)
- **Repository**: [https://github.com/daniloaz/ig-trading-api](https://github.com/daniloaz/ig-trading-api)
- **Documentation**: [README.md](README.md)

## References

- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
- [OWASP Secrets Management](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)
- [The Twelve-Factor App](https://12factor.net/config)
