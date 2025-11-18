# AI Agent Guidelines for IG Trading API

**Last Updated**: November 18, 2025  
**Project**: IG Trading API v0.3.0  
**License**: GPL-3.0-only

---

## 🎯 Purpose

This document defines rules, guidelines, and best practices for AI agents working on this project. Following these rules ensures consistency, quality, and adherence to industry standards.

---

## 📋 Table of Contents

1. [General Rules](#general-rules)
2. [Code Standards](#code-standards)
3. [Documentation Standards](#documentation-standards)
4. [Version Control](#version-control)
5. [Changelog & Release Notes](#changelog--release-notes)
6. [Testing Requirements](#testing-requirements)
7. [Security Guidelines](#security-guidelines)
8. [Communication Style](#communication-style)

---

## 🔧 General Rules

### Project Constraints

1. **ALWAYS identify yourself** at the beginning of EVERY response:
   ```
   **Model: [Your Model Name] (`[model-identifier]`)**
   ```

2. **NEVER make assumptions** - If information is missing, ask the user or search for it in the codebase

3. **ALWAYS work autonomously** - Don't ask for confirmation at every step unless the action is destructive (delete, force push, etc.)

4. **Language**:
   - Code comments: ALWAYS in English
   - Documentation: ALWAYS in English
   - User communication: English (unless specifically requested otherwise)

5. **Date Format**: ALWAYS use ISO 8601 or explicit format (e.g., "November 18, 2025")

---

## 💻 Code Standards

### Rust Best Practices

1. **Follow Rust idioms** and conventions
2. **Use meaningful names** for variables, functions, and types
3. **Write documentation comments** for public APIs using `///`
4. **Implement error handling** properly - no `.unwrap()` in production code
5. **Use clippy** and fix all warnings before committing

### Code Structure

```rust
// GOOD: Clear, documented, error handling
/// Load API configuration from environment variables and config file.
/// 
/// # Errors
/// Returns an error if required environment variables are missing.
pub fn from_env_and_config() -> Result<Self, Box<dyn Error>> {
    // Implementation
}

// BAD: No docs, unwrap in production
pub fn from_env_and_config() -> Self {
    Self::load_env().unwrap()  // ❌ Don't use unwrap
}
```

### Security

1. **NEVER commit secrets** - Use environment variables
2. **ALWAYS validate input** from external sources
3. **Use secure dependencies** - Check for vulnerabilities
4. **Follow OWASP guidelines** for web security

---

## 📚 Documentation Standards

### Project Documentation Structure

The project follows this standard structure:

```
ig_trading_api/
├── README.md                    # Main project documentation
├── CHANGELOG.md                 # Complete version history (primary source)
├── RELEASE_NOTES_v*.md         # Specific release details (optional)
├── LICENSE                      # GPL-3.0 license
└── AGENTS.md                    # This file
```

### README.md Requirements

1. **MUST include**:
   - Project logo and branding
   - Professional badges (shields.io)
   - Clear description
   - Installation instructions
   - Configuration guide (with security best practices)
   - Usage examples
   - API documentation
   - Contributing guidelines
   - License information
   - Contact information

2. **Structure**:
   - Use proper markdown hierarchy (h1 → h2 → h3)
   - Include table of contents for long documents
   - Use code blocks with language identifiers
   - Add clear section separators

---

## 🔄 Version Control

### Semantic Versioning (SemVer)

Follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** (X.0.0): Breaking changes, incompatible API changes
- **MINOR** (0.X.0): New features, backwards-compatible
- **PATCH** (0.0.X): Bug fixes, backwards-compatible

### Version Bump Rules

```rust
0.2.3 → 0.3.0  // Breaking change (security refactoring)
0.3.0 → 0.3.1  // Bug fix (no breaking changes)
0.3.0 → 0.4.0  // New feature (backwards-compatible)
```

### Git Operations - CRITICAL RULES

**⚠️ NEVER do these operations without explicit user authorization:**

1. ❌ **NEVER commit changes** - Always ask first
2. ❌ **NEVER push to remote** - Always ask first
3. ❌ **NEVER create pull requests** - Always ask first
4. ❌ **NEVER merge branches** - Always ask first
5. ❌ **NEVER force push** - Even if asked, warn about dangers
6. ❌ **NEVER delete branches** - Always ask first
7. ❌ **NEVER rebase** - Always ask first
8. ❌ **NEVER amend commits** - Always ask first

**✅ What you CAN do autonomously:**
- Read git status
- View git log
- Check diffs
- View branches
- Inspect commits
- Prepare commit messages (but don't execute)

**🤔 What you MUST ask for:**
```markdown
"I have prepared the following changes. Would you like me to commit them?

Files changed:
- file1.rs
- file2.md

Proposed commit message:
✨ feat(api): add new feature

Should I proceed with this commit? (yes/no)"
```

### Git Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/) with optional emojis.

#### Format

```bash
[emoji] <type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

#### Commit Types

| Type | Emoji | Description | Example |
|------|-------|-------------|---------|
| `feat` | ✨ | New feature | `✨ feat(api): add streaming support` |
| `fix` | 🐛 | Bug fix | `🐛 fix(auth): resolve session timeout` |
| `docs` | 📝 | Documentation | `📝 docs(readme): add installation guide` |
| `style` | 💄 | Code style/formatting | `💄 style(api): format with rustfmt` |
| `refactor` | ♻️ | Code refactoring | `♻️ refactor(config): simplify loading` |
| `perf` | ⚡ | Performance improvement | `⚡ perf(api): optimize request caching` |
| `test` | ✅ | Tests | `✅ test(api): add integration tests` |
| `build` | 👷 | Build system | `👷 build(deps): update tokio to 1.35` |
| `ci` | 💚 | CI configuration | `💚 ci: add GitHub Actions workflow` |
| `chore` | 🔧 | Maintenance | `🔧 chore: update .gitignore` |
| `revert` | ⏪ | Revert changes | `⏪ revert: revert commit abc123` |
| `security` | 🔒 | Security improvements | `🔒 security(config): move secrets to .env` |
| `release` | 🚀 | Release/version bump | `🚀 release: version 0.3.0` |

#### Subject Line Rules

1. **Use imperative mood** - "add feature" not "added feature"
2. **No period at end** - "add feature" not "add feature."
3. **Max 50 characters** for subject line
4. **Capitalize first letter** after type/emoji
5. **Be specific** - "fix login timeout" not "fix bug"

#### Body Rules (Optional)

1. **Wrap at 72 characters**
2. **Explain WHAT and WHY**, not HOW
3. **Use bullet points** for multiple items
4. **Reference issues**: "Fixes #123" or "Closes #456"

#### Footer Rules (Optional)

```bash
# Breaking changes
BREAKING CHANGE: environment variables now required

# Issue references
Fixes #123
Closes #456
Refs #789
```

#### Complete Examples

**Simple commit:**
```bash
✨ feat(config): add environment variable support
```

**With body:**
```bash
🔒 security(config): separate secrets from configuration

Move all sensitive data (API keys, passwords) from config.yaml
to .env file following 12-factor app principles.

- Add dotenvy dependency for .env loading
- Update ApiConfig to load from environment
- Add .env.example template
- Update documentation

BREAKING CHANGE: config.yaml no longer contains secrets
Fixes #42
```

**Bug fix:**
```bash
🐛 fix(api): resolve session timeout in long operations

Sessions were expiring during position creation due to
extended API call duration. Added automatic token refresh
middleware to handle this case.

Fixes #38
```

**Documentation:**
```bash
📝 docs(readme): add professional badges and logo

- Add shields.io badges (version, license, build)
- Include IG logo with proper branding
- Restructure sections following open-source best practices
- Add table of contents
```

**Release:**
```bash
🚀 release: version 0.3.0

Major release with security improvements:
- Environment-based secrets management
- Professional documentation
- 14/14 tests passing

BREAKING CHANGE: configuration system refactored
See RELEASE_NOTES_v0.3.0.md for migration guide
```

#### Commit Message Checklist

Before committing, verify:

- [ ] Type is correct (feat, fix, docs, etc.)
- [ ] Scope is specified (if applicable)
- [ ] Subject is imperative and under 50 chars
- [ ] Body explains WHY (if needed)
- [ ] Breaking changes are documented
- [ ] Issues are referenced
- [ ] Code is tested (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No linter errors (`cargo clippy`)

#### What NOT to Do

❌ **Vague messages:**
```bash
fix stuff
update code
changes
WIP
```

❌ **Multiple unrelated changes in one commit:**
```bash
feat: add streaming + fix auth + update docs
```

❌ **Commit without testing:**
```bash
fix: resolve issue (untested)
```

❌ **Past tense:**
```bash
added feature
fixed bug
```

✅ **Good commits:**
```bash
✨ feat(streaming): add WebSocket support
🐛 fix(auth): handle expired tokens
📝 docs(api): document rate limits
🔒 security: update dependencies
```

#### Emoji Usage (Optional)

Emojis are **optional** but recommended for visual clarity:

- ✅ **Use**: For quick visual identification in git log
- ✅ **Consistent**: Always use the same emoji per type
- ❌ **Don't overuse**: One emoji per commit
- ❌ **Don't mix**: Either use emojis for all commits or none

**Without emojis (also acceptable):**
```bash
feat(config): add environment variable support
fix(auth): resolve session timeout issue
docs(readme): add installation instructions
```

**With emojis (recommended):**
```bash
✨ feat(config): add environment variable support
🐛 fix(auth): resolve session timeout issue
📝 docs(readme): add installation instructions
```

---

## 📝 Changelog & Release Notes

### CHANGELOG.md - The Source of Truth

**ALWAYS follow [Keep a Changelog](https://keepachangelog.com/) format.**

#### Structure

```markdown
# Changelog

## [X.Y.Z] - YYYY-MM-DD

### 🔒 Security (if applicable)
- Security improvements and breaking changes

### ✨ Added
- New features

### 🔧 Changed
- Changes in existing functionality

### 🗑️ Deprecated
- Soon-to-be removed features

### ❌ Removed
- Removed features

### 🐛 Fixed
- Bug fixes

### ✅ Testing
- ALWAYS include test results for major releases
- Format: "X/Y tests passing (Z%)"
- Breakdown by test type

### 📝 Documentation
- Documentation improvements
```

#### Critical Rules for CHANGELOG.md

1. **ALWAYS update** on every release
2. **ALWAYS use exact dates** (YYYY-MM-DD format)
3. **ALWAYS include version number** in brackets: `## [0.3.0]`
4. **ALWAYS include test results** for major/minor releases:
   ```markdown
   ### ✅ Testing
   
   All integration tests pass successfully:
   - **14/14 REST API tests**: 100% PASS
   - **Compilation**: Debug and Release builds successful
   - **Configuration loading**: Verified working correctly
   - **Authentication**: All authentication flows working
   ```

5. **Breaking changes** MUST be clearly marked:
   ```markdown
   ### 🔧 Changed - BREAKING
   ```

6. **Security changes** MUST be at the top:
   ```markdown
   ### 🔒 Security - BREAKING CHANGES
   ```

7. **Migration instructions** for breaking changes:
   ```markdown
   ### 🔄 Migration Required
   
   To upgrade from vX to vY:
   1. Step-by-step instructions
   ```

8. **Include links** at the bottom:
   ```markdown
   ## Links
   
   - **Release Notes**: [RELEASE_NOTES_vX.Y.Z.md](RELEASE_NOTES_vX.Y.Z.md)
   - **Repository**: [URL]
   - **Documentation**: [README.md](README.md)
   ```

### RELEASE_NOTES_vX.Y.Z.md - User-Friendly Version

**Optional but recommended** for major releases.

#### Structure

```markdown
# Release Notes - vX.Y.Z

**Release Date**: Month DD, YYYY
**Type**: Major/Minor/Patch Release
**Focus**: Brief description

## 🎯 Overview
- High-level summary

## 🔒 Security Improvements (if applicable)
- Before/After comparison
- Why this matters

## ✨ New Features
- Feature descriptions

## 📚 Documentation Improvements
- What was improved

## 🔄 Migration Guide
- Quick start steps
- Code examples

## ✅ Testing Results
- Compilation: PASS/FAIL
- Configuration: PASS/FAIL
- Integration Tests: X/Y PASS
- ALWAYS include test details

## 📦 Dependencies
- New/Updated dependencies

## ⚠️ Breaking Changes
- What changed
- Code migration examples

## 🎓 Best Practices Implemented
- Standards compliance

## 🔗 Resources
- Links to CHANGELOG, README, etc.

## 💬 Support
- How to get help

## 🙏 Credits
- Author, License, Website
```

#### Critical Rules for RELEASE_NOTES

1. **ALWAYS include complete test results**:
   ```markdown
   ### Integration Tests: 14/14 PASS ✅
   - ✅ All 14 tests pass successfully (100%)
   - ✅ Session management tests pass
   - ✅ Account operations tests pass
   - List all test categories
   
   **Conclusion**: System is production-ready with 100% test success.
   ```

2. **ALWAYS provide migration examples** for breaking changes
3. **ALWAYS link to CHANGELOG.md** for complete details
4. **ALWAYS use exact dates** (e.g., "November 18, 2025")
5. **NEVER say "some tests pass"** - Be specific with numbers

### When to Create Each

| Change Type | CHANGELOG.md | RELEASE_NOTES |
|-------------|--------------|---------------|
| Bug fix (patch) | ✅ Always | ❌ Optional |
| New feature (minor) | ✅ Always | ⚠️ Recommended |
| Breaking change (major) | ✅ Always | ✅ Always |

---

## 🧪 Testing Requirements

### Before ANY Commit

1. **ALWAYS run tests**: `cargo test`
2. **ALWAYS check compilation**: `cargo build --release`
3. **ALWAYS run clippy**: `cargo clippy`
4. **ALWAYS format code**: `cargo fmt`

### Test Result Reporting

When reporting test results, **ALWAYS be precise**:

```markdown
# ✅ GOOD
Tests: 14/14 PASS (100%)
- REST API: 14/14 ✅
- Unit Tests: 1/1 ✅
- Compilation: Success

# ❌ BAD
"Most tests pass"
"Some tests fail"
"Tests are working"
```

### Integration Tests

1. **ALWAYS verify** after configuration changes
2. **ALWAYS document** test results in CHANGELOG/RELEASE_NOTES
3. **NEVER claim success** without running tests
4. **ALWAYS include** breakdown by test type

---

## 🔐 Security Guidelines

### Secrets Management

1. **NEVER commit secrets** to version control
2. **ALWAYS use `.env` files** for sensitive data
3. **ALWAYS add `.env` to `.gitignore`**
4. **ALWAYS provide `.env.example`** template
5. **ALWAYS document** required environment variables

### Environment Variables

```bash
# ✅ GOOD - In .env file (not in git)
IG_API_KEY=actual_secret_key
IG_USERNAME=username
IG_PASSWORD=password

# ❌ BAD - In config.yaml (might be in git)
api_key: "actual_secret_key"
username: "username"
password: "password"
```

### Configuration Structure

```
.env                  # Secrets (NEVER in git)
.env.example          # Template (safe for git)
config.yaml           # Non-sensitive settings (safe for git)
config.default.yaml   # Template (safe for git)
```

### Security Checklist

Before any release:

- [ ] All secrets in `.env`
- [ ] `.env` in `.gitignore`
- [ ] `.env.example` provided
- [ ] `config.yaml` has no secrets
- [ ] README documents security practices
- [ ] CHANGELOG mentions security changes

---

## 💬 Communication Style

### With Users

1. **Be clear and concise** - No unnecessary jargon
2. **Be specific** - Use exact numbers and metrics
3. **Be honest** - Don't claim things that aren't verified
4. **Be helpful** - Provide context and explanations
5. **Use English** - Code and documentation in English

### Documentation Writing

1. **Use active voice**: "Configure the API" not "The API should be configured"
2. **Be specific**: "Run `cargo test`" not "Run the tests"
3. **Provide examples**: Show, don't just tell
4. **Use proper markdown**: Code blocks, lists, headers
5. **Add emojis sparingly**: Only for visual organization (✅ ❌ ⚠️)

### Error Reporting

```markdown
# ✅ GOOD
Error: 7 tests failed due to API session timeout (HTTP 401)
Root cause: Long-running operations exceed session TTL
Impact: Does NOT affect configuration system

# ❌ BAD
Some tests didn't work
There were problems
Tests failed
```

---

## 🎓 Best Practices Summary

### Always Do

✅ Update CHANGELOG.md on every release  
✅ Include exact dates everywhere  
✅ Run and document all tests  
✅ Use semantic versioning  
✅ Write clear commit messages  
✅ Keep secrets in environment variables  
✅ Provide migration guides for breaking changes  
✅ Follow Rust best practices  
✅ Document all public APIs  

### Never Do

❌ Commit secrets to git  
❌ Use vague test descriptions  
❌ Skip testing before commits  
❌ Make breaking changes in patch versions  
❌ Use `.unwrap()` in production  
❌ Leave code undocumented  
❌ Guess or assume - verify  
❌ Mix documentation languages  

---

## 📚 References

### Standards

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [The Twelve-Factor App](https://12factor.net/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)

### Rust

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)

### Security

- [OWASP Secrets Management](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)
- [CIS Controls](https://www.cisecurity.org/controls)

---

## 🔄 Updating This Document

When updating AGENTS.md:

1. **Update "Last Updated" date** at the top
2. **Update version number** if project version changed
3. **Document why** the rules were added/changed
4. **Keep it concise** - Only essential rules
5. **Provide examples** for complex rules

---

**Project**: IG Trading API  
**Maintainer**: Daniel López Azaña  
**License**: GPL-3.0-only  
**Repository**: https://github.com/daniloaz/ig-trading-api

---

*This document ensures consistency and quality across all AI-assisted development work on this project.*

