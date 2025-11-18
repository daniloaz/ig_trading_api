# Release Notes - v0.3.0

**Release Date**: November 18, 2025  
**Type**: Major Release (Breaking Changes)  
**Focus**: Security Improvements & Professional Documentation

---

## 🎯 Overview

Version 0.3.0 introduces critical security improvements by implementing proper secrets management following industry best practices. This is a **breaking change** that requires migration from the previous configuration format.

---

## 🔒 Security Improvements (BREAKING)

### What Changed

**Before v0.2.3:**
```yaml
# config.yaml - ALL IN ONE FILE (insecure!)
ig_trading_api:
  api_key: "secret123..."     # ❌ Exposed in git
  username: "user"            # ❌ Exposed in git
  password: "pass"            # ❌ Exposed in git
  auto_login: true
```

**After v0.3.0:**
```bash
# .env - Secrets (NOT in git) ✅
IG_API_KEY=secret123...
IG_USERNAME=user
IG_PASSWORD=pass
```

```yaml
# config.yaml - Settings only (safe for git) ✅
ig_trading_api:
  auto_login: true
  logger: "StdLogs"
```

### Why This Matters

1. **Prevents Accidental Exposure** - Secrets can't be committed to git
2. **Compliance** - Meets OWASP, 12-factor app, and PCI DSS standards
3. **Environment Management** - Easy to have different credentials for dev/staging/prod
4. **Secret Rotation** - Change credentials without touching code
5. **Audit Trail** - Clear separation of configuration vs secrets

---

## ✨ New Features

### Environment Variable Support
- Automatic `.env` file loading via `dotenvy` crate
- Clear error messages for missing variables
- Optional `.env` file (can use system environment variables)

### New API Methods
```rust
// Explicit loading (recommended)
let config = ApiConfig::from_env_and_config()?;

// Helper for loading .env
ApiConfig::load_env()?;

// Default still works (uses from_env_and_config internally)
let config = ApiConfig::default();
```

### Documentation Files
- `.env.example` - Template for environment variables
- `MIGRATION_GUIDE.md` - Step-by-step migration from v0.2.3
- `SECURITY_IMPROVEMENTS.md` - Detailed security analysis
- `TEST_RESULTS.md` - Test results and analysis
- `CHANGELOG.md` - Complete version history

---

## 📚 Documentation Improvements

### Professional README.md
- ✅ IG logo and branding
- ✅ Professional shields.io badges
- ✅ Comprehensive table of contents
- ✅ Security-focused configuration guide
- ✅ Clear examples and usage instructions
- ✅ Contributing guidelines
- ✅ Proper licensing information

### Compliance Documentation
- OWASP Top 10 compliance
- 12-Factor App methodology
- CIS Controls adherence
- NIST guidelines alignment

---

## 🔄 Migration Guide

### Quick Start

1. **Copy the template:**
   ```bash
   cp .env.example .env
   ```

2. **Move your secrets to `.env`:**
   ```bash
   IG_API_KEY=your_actual_api_key
   IG_USERNAME=your_username
   IG_PASSWORD=your_password
   IG_ACCOUNT_NUMBER_DEMO=XXXXX
   IG_ACCOUNT_NUMBER_LIVE=XXXXX
   IG_BASE_URL_DEMO=https://demo-api.ig.com/gateway/deal
   IG_BASE_URL_LIVE=https://api.ig.com/gateway/deal
   IG_EXECUTION_ENVIRONMENT=DEMO
   ```

3. **Update `config.yaml`** (remove all secrets):
   ```yaml
   ig_trading_api:
     auto_login: true
     logger: "StdLogs"
     session_version: 2
     streaming_api_max_connection_attempts: 3
   ```

4. **No code changes required!** (unless you want to use the new explicit methods)

For complete changelog, see [CHANGELOG.md](CHANGELOG.md)

---

## ✅ Testing Results

### Compilation: PASS
- ✅ Debug build successful
- ✅ Release build successful
- ✅ No linter errors

### Configuration: PASS
- ✅ Environment variable loading works
- ✅ Config file loading works
- ✅ Authentication successful
- ✅ Session management functional

### Integration Tests: 14/14 PASS ✅
- ✅ All 14 tests pass successfully (100%)
- ✅ Session management tests pass
- ✅ Account operations tests pass
- ✅ Market operations tests pass
- ✅ Position management tests pass
- ✅ Working orders tests pass
- ✅ Historical data tests pass

**Conclusion**: Configuration system is **production-ready** and fully tested with 100% test success rate.

---

## 📦 Dependencies

### New Dependencies
- `dotenvy = "0"` - For loading `.env` files

### Updated Dependencies
None (all existing dependencies remain the same)

---

## ⚠️ Breaking Changes

### Configuration Loading

**Old Way (v0.2.3):**
```rust
// Loaded everything from config.yaml
let config = ApiConfig::default();
```

**New Way (v0.3.0):**
```rust
// Loads secrets from .env, settings from config.yaml
let config = ApiConfig::default(); // Still works!

// Or explicitly
let config = ApiConfig::from_env_and_config()?;
```

### Required Environment Variables

These environment variables are **required** in v0.3.0:
- `IG_API_KEY`
- `IG_USERNAME`
- `IG_PASSWORD`
- `IG_ACCOUNT_NUMBER_DEMO`
- `IG_ACCOUNT_NUMBER_LIVE`
- `IG_BASE_URL_DEMO`
- `IG_BASE_URL_LIVE`
- `IG_EXECUTION_ENVIRONMENT`

Optional:
- `IG_ACCOUNT_NUMBER_TEST`

---

## 🎓 Best Practices Implemented

1. ✅ **Separation of Concerns** - Secrets vs Configuration
2. ✅ **Environment Variables** - 12-factor app methodology
3. ✅ **Never Commit Secrets** - `.env` in `.gitignore`
4. ✅ **Template Files** - `.env.example` for documentation
5. ✅ **Clear Error Messages** - Helpful panics for missing variables
6. ✅ **Professional Documentation** - Industry-standard README
7. ✅ **Version Control Safety** - Protected secrets
8. ✅ **Easy Onboarding** - Clear setup instructions

---

## 🔗 Resources

- **Full Changelog**: [CHANGELOG.md](CHANGELOG.md) - Complete version history
- **Repository**: [https://github.com/daniloaz/ig-trading-api](https://github.com/daniloaz/ig-trading-api)
- **Documentation**: [README.md](README.md) - Main documentation

---

## 💬 Support

If you encounter any issues during migration:
1. Verify all required environment variables are set in `.env`
2. Ensure `.env` is in the project root (same directory as `config.yaml`)
3. Check that `config.yaml` only contains non-sensitive settings
4. See [CHANGELOG.md](CHANGELOG.md) for complete details
5. Open an issue on [GitHub](https://github.com/daniloaz/ig-trading-api/issues)

---

## 🙏 Credits

- **Author**: Daniel López Azaña
- **License**: GPL-3.0-only
- **Website**: [www.daniloaz.com](https://www.daniloaz.com/en/)

---

**🎉 Thank you for using IG Trading API!**

Please ⭐ star the repository if you find it useful!
