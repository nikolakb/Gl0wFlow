# GlowFlow Release Checklist

This checklist is the shortest safe path for publishing a new GlowFlow release with packaged artifacts.

Current target release line:

- `GlowFlow v1.1.2`
- `GlowScript v1.1.2`

## 1. Verify the local tree

Run:

```bash
cargo test
```

Expected result:

- all tests pass

Optional local package verification on macOS ARM:

```bash
./scripts/package-release.sh 1.1.2 aarch64-apple-darwin pkg
```

## 2. Commit the release state

Run:

```bash
git add .
git commit -m "Prepare v1.1.2 release packaging"
```

## 3. Push the main branch

Run:

```bash
git push -u origin main
```

Expected result:

- GitHub receives the release workflow and packaging script updates

## 4. Trigger release packaging

Choose one of these paths.

### Option A: run the workflow manually

On GitHub:

1. Open the repository
2. Open the `Actions` tab
3. Open `release-packages`
4. Click `Run workflow`

### Option B: trigger by version tag

Run:

```bash
git tag v1.1.2
git push origin v1.1.2
```

Expected result:

- GitHub Actions runs the release matrix automatically

## 5. Confirm the generated artifacts

The workflow should produce:

- `glowflow-v1.1.2-x86_64-unknown-linux-gnu.tar.gz`
- `glowflow-v1.1.2-x86_64-pc-windows-msvc.zip`
- `glowflow-v1.1.2-x86_64-pc-windows-msvc.msi`
- `glowflow-v1.1.2-aarch64-apple-darwin.pkg`

## 6. Publish a GitHub Release

On GitHub:

1. Open `Releases`
2. Create a new release for tag `v1.1.2`
3. Upload the generated artifacts
4. Paste the release notes

Suggested title:

```text
GlowFlow v1.1.2 / GlowScript v1.1.2
```

## 7. Recommended release notes

```text
GlowFlow v1.1.2 extends the language with advanced context-control built-ins while carrying forward first-class error recovery, HTTPS webhook support, and tighter runtime proof.

Highlights
- Added try / catch / throw / recover
- Added HTTPS webhook serving to glow serve
- Improved runtime reliability for automation and webhook flows
- Added focused regression coverage for the new language and runtime behavior
- Added new examples for error recovery, HTTPS webhooks, and more robust automation patterns

This release improves execution, recovery, and deployable automation without pretending the surrounding ecosystem is finished.
```

## 8. Recommended public links

Use these in the GitHub Release body, repo README, or Product Hunt comments:

- Landing page: `https://nikolakb.github.io/Gl0wFlow/`
- Terminal demo: `https://nikolakb.github.io/Gl0wFlow/terminal-demo.html`
- Demo page: `https://nikolakb.github.io/Gl0wFlow/demo.html`
- Semantics reference: `https://nikolakb.github.io/Gl0wFlow/semantics.html`
- Packages page: `https://nikolakb.github.io/Gl0wFlow/packages.html`

## 9. Final release sanity checks

Before announcing publicly, confirm:

- the Actions run is green
- all 4 package artifacts are attached
- the Windows MSI downloads correctly
- the Linux tarball extracts cleanly
- the macOS `.pkg` installs correctly
- GitHub Pages renders the docs without broken links
- the terminal demo page loads and replays

## 10. What this release claims

Safe claims:

- GlowScript is a real scripting language for AI automation
- GlowFlow has a real CLI, runtime, and native build path
- the project ships packaged binaries/installers for Windows, Linux, and macOS

Claims to avoid:

- fully production-hardened across every environment
- complete MCP ecosystem
- general-purpose language replacement
