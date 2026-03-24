# GlowFlow Runtime

The current runtime includes:

- local interpretation for core Glow statements
- file reads and writes
- JSON and CSV helpers
- simple HTTP over `http://`
- local webhook serving
- schedule registration in MVP mode
- AI execution through `command`, `mock`, `openai`, and `ollama` providers
- MCP-style filesystem tools and user-defined Glow tools

Native transpilation is available, but the interpreter remains the most complete execution path today.

## Server-Side Performance Guidance

For the current MVP, "fast on a server" should be interpreted as low language
overhead rather than unrealistic end-to-end latency claims.

Good target ranges for GlowFlow are:

- CLI startup for a small script: under `100ms`
- parse plus semantic check: under `50ms`
- local interpreter execution for simple logic: under `100ms`
- simple webhook path without external AI or API calls: `50ms` to `300ms`
- local automation runs without slow external dependencies: under `1s`

In practice, the largest delays usually come from:

- AI provider calls
- HTTP APIs
- file I/O
- webhook waiting
- external tools and MCP integrations

That means the runtime goal is straightforward: the language should feel
near-instant, and the visible latency should mostly come from the automation
work itself.
