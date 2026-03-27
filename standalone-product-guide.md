# GlowFlow as a Standalone Product
## How to ship GlowFlow with zero Rust dependency for end users

This document answers one question: **how do you turn GlowFlow from a Rust project into a product that someone can download and run without knowing what Rust is?**

---

## The core problem

Right now, running GlowFlow requires:

1. A Rust toolchain installed (`rustup`, `cargo`, `rustc`)
2. Building from source with `cargo build --release`
3. Understanding what a Cargo workspace is

That is fine for developers evaluating the language. It is a hard stop for the actual target audience — operations people, growth teams, founders, and non-developers who want to automate AI workflows.

The good news: Rust makes this completely solvable. The solution is **precompiled native binaries**, and Rust's cross-compilation story is one of the best in any language ecosystem.

---

## Step 1 — Build a single self-contained binary

Rust compiles to a single static binary by default. The goal is a `glow` executable that:

- Has no runtime dependencies (no Rust, no Python, no Node)
- Works on the target OS out of the box
- Is small enough to distribute easily

### How to do it

In `Cargo.toml`, make sure you are building a binary target:

```toml
[[bin]]
name = "glow"
path = "src/main.rs"
```

Build for release (optimized, smaller):

```bash
cargo build --release
```

The output is `target/release/glow` (Linux/macOS) or `target/release/glow.exe` (Windows). This single file is your product.

### Reduce binary size

Add this to `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"       # optimize for size
lto = true            # link-time optimization
codegen-units = 1     # slower build, smaller output
strip = true          # strip debug symbols
panic = "abort"       # smaller panic handler
```

With these settings a typical CLI binary can drop from 10–20MB to 2–5MB.

---

## Step 2 — Cross-compile for all platforms from one machine

Rust's cross-compilation lets you build for Linux, macOS, and Windows from a single CI environment.

### Target triples you need

| Platform | Target triple |
|---|---|
| Linux x86_64 | `x86_64-unknown-linux-musl` |
| Linux ARM64 | `aarch64-unknown-linux-musl` |
| macOS Intel | `x86_64-apple-darwin` |
| macOS Apple Silicon | `aarch64-apple-darwin` |
| Windows x86_64 | `x86_64-pc-windows-gnu` |

### Add targets

```bash
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
```

### Build for a specific target

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

### Use `cross` for Linux musl builds (recommended)

The `cross` tool handles the musl toolchain automatically:

```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl
```

musl builds produce fully static binaries that run on any Linux distribution without glibc version concerns. This is the recommended Linux distribution format.

---

## Step 3 — Automate with GitHub Actions

Add this workflow to `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
            binary: glow
            archive: tar.gz

          - target: aarch64-unknown-linux-musl
            os: ubuntu-latest
            binary: glow
            archive: tar.gz

          - target: x86_64-apple-darwin
            os: macos-latest
            binary: glow
            archive: tar.gz

          - target: aarch64-apple-darwin
            os: macos-latest
            binary: glow
            archive: tar.gz

          - target: x86_64-pc-windows-gnu
            os: ubuntu-latest
            binary: glow.exe
            archive: zip

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross (Linux)
        if: matrix.os == 'ubuntu-latest'
        run: cargo install cross

      - name: Build (Linux with cross)
        if: matrix.os == 'ubuntu-latest'
        run: cross build --release --target ${{ matrix.target }}

      - name: Build (macOS native)
        if: matrix.os == 'macos-latest'
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package (tar.gz)
        if: matrix.archive == 'tar.gz'
        run: |
          mkdir dist
          cp target/${{ matrix.target }}/release/${{ matrix.binary }} dist/
          tar -czf glowflow-${{ github.ref_name }}-${{ matrix.target }}.tar.gz -C dist .

      - name: Package (zip)
        if: matrix.archive == 'zip'
        run: |
          mkdir dist
          cp target/${{ matrix.target }}/release/${{ matrix.binary }} dist/
          zip glowflow-${{ github.ref_name }}-${{ matrix.target }}.zip dist/${{ matrix.binary }}

      - name: Upload to GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: glowflow-*
```

Push a tag and GitHub Actions builds and uploads all platform binaries to a GitHub Release automatically. No manual steps.

---

## Step 4 — Write an installer script

Users should be able to install GlowFlow with a single curl command. This is now the expected standard for CLI tools.

```bash
#!/usr/bin/env bash
# installer.sh — installs the glow CLI

set -e

REPO="nikolakb/Gl0wFlow"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="glow"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  x86_64)  ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

case "$OS" in
  linux)  TARGET="${ARCH}-unknown-linux-musl" ;;
  darwin) TARGET="${ARCH}-apple-darwin" ;;
  *) echo "Unsupported OS: $OS" && exit 1 ;;
esac

# Get latest version
VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

ARCHIVE="glowflow-${VERSION}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

echo "Installing GlowFlow ${VERSION} for ${TARGET}..."

curl -fsSL "$URL" | tar -xz -C /tmp
chmod +x "/tmp/${BINARY_NAME}"
mv "/tmp/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"

echo "Installed: $(glow --version)"
echo "Run: glow help"
```

Publish this as `installer.sh` in the repo. Users install with:

```bash
curl -fsSL https://raw.githubusercontent.com/nikolakb/Gl0wFlow/main/installer.sh | bash
```

---

## Step 5 — Platform-native packages (optional but high value)

### macOS — Homebrew tap

Create a separate repo `nikolakb/homebrew-glowflow` with a formula:

```ruby
# Formula/glowflow.rb
class Glowflow < Formula
  desc "Plain-English AI automation powered by GlowScript"
  homepage "https://github.com/nikolakb/Gl0wFlow"
  version "1.1.2"

  on_macos do
    on_arm do
      url "https://github.com/nikolakb/Gl0wFlow/releases/download/v1.1.2/glowflow-v1.1.2-aarch64-apple-darwin.tar.gz"
      sha256 "<sha256>"
    end
    on_intel do
      url "https://github.com/nikolakb/Gl0wFlow/releases/download/v1.1.2/glowflow-v1.1.2-x86_64-apple-darwin.tar.gz"
      sha256 "<sha256>"
    end
  end

  def install
    bin.install "glow"
  end

  test do
    assert_match "GlowFlow", shell_output("#{bin}/glow --version")
  end
end
```

Users install with:

```bash
brew tap nikolakb/glowflow
brew install glowflow
```

### Linux — direct binary or apt/deb (later)

For now, the musl binary + installer script is sufficient. A `.deb` package is worth considering once there is user adoption.

### Windows — winget or direct zip

The zip release is workable for Windows. A `winget` package requires a submission to the winget community repo — worth doing after the product is more established.

---

## Step 6 — Remove binaries from the repo root

The `.pkg`, `.tar.gz`, and `.zip` files currently committed to the repo root should be removed. They belong in GitHub Releases, not in the repository tree. This also significantly improves the perceived language breakdown (removing binary blobs makes the Rust source percentage more representative).

```bash
git rm glowflow-v1.0.1-*.pkg
git rm glowflow-v1.0.1-*.zip
git rm glowflow-v1.0.1-*.tar.gz
git commit -m "move release artifacts to GitHub Releases"
```

---

## Summary: the distribution stack

| Layer | Tool | Effort |
|---|---|---|
| Build automation | GitHub Actions | Low — one workflow file |
| Linux static binary | musl + `cross` | Low — already cross-compiling |
| macOS universal binary | `cargo build` per target | Low |
| Windows binary | `x86_64-pc-windows-gnu` | Low |
| One-line installer | `installer.sh` | Low |
| macOS package manager | Homebrew tap | Medium |
| Linux package manager | `.deb` / apt | High (later) |
| Windows package manager | winget | Medium (later) |

The first four rows — static Linux binary, macOS binary, Windows binary, and installer script — are enough to call GlowFlow a properly distributed product. GitHub Actions makes all four automatic on every tag push.

---

## The dependency question: what stays, what goes

Currently GlowFlow's `Cargo.toml` has two production dependencies:

```toml
rustls = "0.21"
rustls-pemfile = "1.0"
```

These are for HTTPS webhook serving. They are good choices — `rustls` is a pure-Rust TLS implementation with no C dependency, which means the resulting binary is still fully static and musl-compatible.

When adding new capabilities, prefer:

- **Pure Rust crates** that compile to static code (no C FFI, no OpenSSL)
- **`serde` + `serde_json`** for JSON — standard, well-audited, musl-friendly
- **`tokio`** if async is needed — but consider whether the interpreter model actually needs it
- **Avoid `openssl-sys`** — it requires system OpenSSL and breaks static musl builds

The goal is to stay in a world where `cross build --release --target x86_64-unknown-linux-musl` just works with no system dependencies.

---

## Suggested Cargo.toml additions

```toml
[dependencies]
rustls        = "0.23"          # upgrade: 0.21 is old
rustls-pemfile = "2.0"          # keep in sync with rustls
serde         = { version = "1", features = ["derive"] }
serde_json    = "1"
clap          = { version = "4", features = ["derive"] }  # CLI argument parsing

[profile.release]
opt-level     = "z"
lto           = true
codegen-units = 1
strip         = true
panic         = "abort"
```

`clap` replaces manual argument parsing with a proper CLI framework that auto-generates `--help` output. That is table stakes for a product CLI.
