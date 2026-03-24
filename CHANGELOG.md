# Changelog

## v1.0.1

- Added HTTPS webhook serving with `glow serve ... --https --cert ... --key ...`
- Added language-level error handling with `try`, `catch`, `throw`, and postfix `recover`
- Added TLS configuration loading with beginner-readable diagnostics for missing or malformed certificate files
- Added new examples for error recovery, secure webhooks, and more robust automation flows
- Added focused `v1.0.1` regression tests on top of the retained mother end-to-end scenario
- Kept the interpreter/native parity proof path intact while extending the language surface

## v1.0.0

- First public release line for GlowFlow and GlowScript
- Plain-English automation language with parser, formatter, semantics, interpreter, CLI, and native build path
