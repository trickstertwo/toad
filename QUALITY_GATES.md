# Quality Gates

> Ensuring world-class code quality for Toad AI Coding Terminal

## Overview

Every feature, module, and pull request MUST pass these quality gates before being merged. This document defines the specific checks, tools, and thresholds required.

## Gate 1: Code Quality & Style

### Clippy (Strict Mode)
```bash
# Must pass with ZERO warnings
cargo clippy --all-targets --all-features -- -D warnings

# Pedantic mode for new code
cargo clippy --all-targets --all-features -- -W clippy::pedantic
```

**Thresholds**:
- ✅ 0 warnings
- ✅ 0 errors
- ✅ All clippy lints enabled

### Formatting
```bash
# Must pass without changes
cargo fmt --check
```

**Thresholds**:
- ✅ 100% formatted
- ✅ rustfmt.toml adhered to

### Unsafe Code
```bash
# Must justify any unsafe blocks
cargo geiger
```

**Thresholds**:
- ✅ 0 unsafe blocks (preferred)
- ✅ All unsafe code documented with safety invariants
- ✅ Justification required for any unsafe usage

## Gate 2: Testing

### Unit Tests
```bash
# All tests must pass
cargo test --lib

# Run with output
cargo test --lib -- --nocapture
```

**Thresholds**:
- ✅ 100% passing
- ✅ Minimum 80% code coverage
- ✅ All public functions tested
- ✅ Edge cases covered

### Integration Tests
```bash
# All integration tests must pass
cargo test --test '*'
```

**Thresholds**:
- ✅ 100% passing
- ✅ Key user workflows tested
- ✅ Error paths tested

### Doc Tests
```bash
# All documentation examples must work
cargo test --doc
```

**Thresholds**:
- ✅ 100% passing
- ✅ All code examples in docs tested
- ✅ No outdated examples

### Test Coverage
```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Or use llvm-cov
cargo llvm-cov --html
```

**Thresholds**:
- ✅ Overall coverage ≥ 80%
- ✅ Core modules ≥ 90%
- ✅ Critical paths 100%

## Gate 3: Documentation

### Rustdoc Completeness
```rust
// In lib.rs or main module
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
```

```bash
# Generate docs and check for warnings
cargo doc --no-deps --document-private-items
```

**Thresholds**:
- ✅ All public items documented
- ✅ All modules have module-level docs
- ✅ Examples for all public functions
- ✅ 0 documentation warnings

### Documentation Quality Checklist
- [ ] Module purpose clearly explained
- [ ] Public API documented with examples
- [ ] Error conditions documented
- [ ] Panics documented (or none exist)
- [ ] Safety requirements for unsafe code
- [ ] Complexity notes for non-obvious code

## Gate 4: Performance

### Binary Size
```bash
# Release build size check
cargo build --release
strip target/release/toad
ls -lh target/release/toad
```

**Thresholds**:
- ✅ Stripped binary ≤ 10 MB
- ✅ No regression > 5% per PR

### Compile Time
```bash
# Track compile time
cargo clean
time cargo build --release
```

**Thresholds**:
- ✅ Release build ≤ 60 seconds (on CI)
- ✅ Incremental rebuild ≤ 5 seconds
- ✅ No regression > 10% per PR

### Runtime Performance
```bash
# Startup time
hyperfine './target/release/toad --version'

# Run benchmarks
cargo bench
```

**Thresholds**:
- ✅ Startup time ≤ 100ms
- ✅ UI render time ≤ 16ms (60 FPS)
- ✅ No benchmark regression > 5%

### Memory Usage
```bash
# Profile memory usage
valgrind --tool=massif ./target/release/toad

# Or use heaptrack on Linux
heaptrack ./target/release/toad
```

**Thresholds**:
- ✅ Idle memory ≤ 50 MB
- ✅ Peak memory ≤ 200 MB
- ✅ No memory leaks

## Gate 5: Dependencies

### Dependency Audit
```bash
# Check for security vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated
```

**Thresholds**:
- ✅ 0 known vulnerabilities
- ✅ 0 unmaintained dependencies
- ✅ Dependencies updated quarterly

### Unused Dependencies
```bash
# Check for unused dependencies
cargo udeps
```

**Thresholds**:
- ✅ 0 unused dependencies in Cargo.toml
- ✅ All features justified

### License Compliance
```bash
# Check dependency licenses
cargo license
```

**Thresholds**:
- ✅ All dependencies MIT or Apache-2.0
- ✅ No GPL dependencies
- ✅ License file up-to-date

## Gate 6: Cross-Platform

### Platform Testing Matrix

#### Required Platforms
- **Linux**: Ubuntu 22.04+ (x86_64)
- **macOS**: macOS 12+ (x86_64, aarch64)
- **Windows**: Windows 10+ (x86_64)

#### Optional Platforms
- Linux aarch64 (Raspberry Pi, ARM servers)
- FreeBSD (best effort)

### Platform-Specific Tests
```bash
# Run platform-specific tests
cargo test --features platform-specific

# Cross-compilation check
cargo build --target x86_64-pc-windows-gnu
cargo build --target aarch64-apple-darwin
```

**Thresholds**:
- ✅ All required platforms pass CI
- ✅ No platform-specific panics
- ✅ Terminal detection works on all platforms

## Gate 7: Error Handling

### Error Handling Checklist
- [ ] No unwrap() in production code
- [ ] No expect() without justification
- [ ] All Result types handled
- [ ] All Option types handled safely
- [ ] Errors provide actionable messages
- [ ] Panic hook restores terminal state

### Error Handling Tests
```rust
#[test]
fn test_error_handling() {
    // Test all error paths
    assert!(func_that_fails().is_err());
}
```

**Thresholds**:
- ✅ All error paths tested
- ✅ 0 unwrap() calls in src/ (tests are OK)
- ✅ Terminal always restored on panic

## Gate 8: Accessibility

### Accessibility Checklist
- [ ] Keyboard-only navigation works
- [ ] Color contrast ratios ≥ 4.5:1
- [ ] No information conveyed by color alone
- [ ] Screen reader compatible (where possible)
- [ ] Configurable UI (size, colors, animations)

**Thresholds**:
- ✅ WCAG 2.1 AA compliance (where applicable)
- ✅ Works in 80x24 terminal minimum
- ✅ Graceful degradation on limited terminals

## Gate 9: Code Organization

### Module Organization Checklist
- [ ] Clear module boundaries
- [ ] No circular dependencies
- [ ] Proper visibility (pub/private)
- [ ] Logical file structure
- [ ] < 500 LOC per file (guideline)

### Code Complexity
```bash
# Check cyclomatic complexity
cargo clippy -- -W clippy::cognitive_complexity
```

**Thresholds**:
- ✅ Functions < 50 LOC
- ✅ Cyclomatic complexity < 10
- ✅ No deeply nested code (> 4 levels)

## Gate 10: Security

### Security Checklist
- [ ] No hardcoded secrets
- [ ] Input validation on all user input
- [ ] Path traversal prevention
- [ ] Command injection prevention
- [ ] No eval() equivalents

### Security Tools
```bash
# Security audit
cargo audit

# Check for common vulnerabilities
cargo geiger
```

**Thresholds**:
- ✅ 0 known vulnerabilities
- ✅ All external input validated
- ✅ Security review for sensitive code

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Quality Gates

on: [push, pull_request]

jobs:
  quality-gates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Gate 1 - Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Gate 1 - Format
        run: cargo fmt --check

      - name: Gate 2 - Tests
        run: cargo test --all-features

      - name: Gate 2 - Coverage
        run: cargo tarpaulin --out Xml

      - name: Gate 3 - Docs
        run: cargo doc --no-deps

      - name: Gate 4 - Build
        run: cargo build --release

      - name: Gate 5 - Audit
        run: cargo audit

      - name: Upload Coverage
        uses: codecov/codecov-action@v3

  cross-platform:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - name: Test
        run: cargo test
```

## Feature-Specific Gates

### New Feature Checklist

When adding a new feature, ensure:

- [ ] **Design doc** written and reviewed
- [ ] **API designed** with public interface defined
- [ ] **Tests written** BEFORE implementation (TDD)
- [ ] **Implementation** passes all quality gates
- [ ] **Documentation** complete with examples
- [ ] **Performance** benchmarked
- [ ] **Integration** tested with existing features
- [ ] **Accessibility** considered
- [ ] **Error handling** comprehensive
- [ ] **Code review** completed
- [ ] **User docs** updated
- [ ] **ROADMAP** updated

## Quality Metrics Dashboard

Track these metrics over time:

| Metric | Target | Current |
|--------|--------|---------|
| Test Coverage | ≥ 80% | 📊 Track |
| Clippy Warnings | 0 | ✅ 0 |
| Binary Size | ≤ 10 MB | 📊 Track |
| Startup Time | ≤ 100ms | 📊 Track |
| Dependencies | Minimal | 📊 Track |
| Documentation | 100% | 📊 Track |
| Security Audit | 0 issues | ✅ 0 |

## Enforcement

### Pre-commit Hooks
```bash
# Install pre-commit hooks
cp scripts/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### Pre-commit checks:
1. cargo fmt
2. cargo clippy
3. cargo test (fast tests only)

### Pull Request Requirements
- ✅ All CI checks pass
- ✅ Code review approved
- ✅ Documentation updated
- ✅ Tests added/updated
- ✅ No merge conflicts

## Exemptions

Quality gates can be bypassed only with:
1. **Justification** - Documented reason
2. **Approval** - Maintainer approval
3. **Tracking** - Issue created to address later

Example:
```rust
// SAFETY: This is safe because XYZ invariant is maintained
// See issue #123 for planned refactoring
#[allow(clippy::unwrap_used)]
let value = option.unwrap();
```

## Continuous Improvement

Quality gates are living documents:
- Review quarterly
- Update based on project needs
- Add gates as project matures
- Remove gates that don't add value

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rustdoc Book](https://doc.rust-lang.org/rustdoc/)
