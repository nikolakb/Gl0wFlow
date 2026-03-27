# GlowFlow Runtime

The GlowFlow runtime is the execution layer that runs GlowScript files. It has two paths: the interpreter and the native build path.

---

## Execution paths

### `glow run` — Interpreter

The interpreter is the primary execution path and the semantic reference for the language. It executes `.glow` files directly without a compilation step.

Use `glow run` for development, testing, and local automation.

### `glow build` — Native binary

`glow build` transpiles a GlowScript file to Rust source and compiles it to a native binary using the local `rustc` toolchain. If `rustc` is not present, it outputs the generated `.rs` file instead.

Native binaries are self-contained and carry no Glow runtime dependency at execution time. This is the deployment path for production automation and distribution.

---

## What the runtime supports today

**Language execution**
- Full interpreter for the core GlowScript language
- Native transpilation for the supported language subset

**Data and I/O**
- File reads and writes (`read file`, `save`, `append`)
- JSON loading and saving (`load json`, `save json`)
- CSV loading and saving (`load csv`, `save csv`)

**HTTP**
- Plain HTTP client over `http://` for `get`, `post`, `put`, `delete`
- HTTPS webhook serving with `--https --cert --key` flags
- Per-request payload persistence with unique saved files

**Webhooks**
- Local webhook server via `glow serve`
- HTTPS webhook server with TLS certificate configuration
- `read json body` populates the `body` variable inside webhook handlers

**AI execution**
- `ask ai` with provider configuration
- Structured AI forms: `extract`, `classify`, `summarize`, `rewrite`, `decide`
- Supported providers: `openai`, `ollama`, `command`, `mock`
- Falls back to `GLOW_AI_COMMAND` environment variable when no provider is set
- Returns a stub when neither is configured

**Tools and MCP**
- `use mcp filesystem` exposes filesystem-style tool calls
- User-defined `tool` blocks callable via `call tool`
- `export mcp tool` makes tools available through the MCP surface
- `list tools` returns available tool names at runtime

**Error handling**
- `try` / `catch` / `throw` for structured error flow
- `recover` postfix for expression-level fallback

**Context control**
- `calculate tokens` for deterministic token estimation
- `collapse` for structural text compaction
- `compress` for policy-driven context reduction with retention targeting

**Scheduling**
- `every` registers a schedule and runs the body immediately in MVP mode
- Full daemon-grade background scheduler is planned but not yet claimed

---

## Runtime environment variables

| Variable | Effect |
|---|---|
| `GLOW_AI_COMMAND` | Shell command used as the AI backend when no provider block is set |
| `OPENAI_API_KEY` | API key for the `openai` provider |
| `OLLAMA_HOST` | Host override for the `ollama` provider |

---

## Performance targets

These are language-overhead targets, not end-to-end guarantees. Real latency in production is almost always dominated by AI provider calls, HTTP APIs, or file I/O — not by GlowFlow itself.

| Operation | Target |
|---|---|
| CLI startup, small script | < 100ms |
| Parse + semantic check | < 50ms |
| Interpreter, simple logic | < 100ms |
| Webhook response, no external calls | 50–300ms |
| Full local automation, no slow dependencies | < 1s |

---

## Optional telemetry companion

The repository includes a FastAPI telemetry proxy at `telemetry/fastapi_proxy.py`. This is an optional side-channel service for collecting runtime metrics. No new syntax is required — use the built-in `post` expression:

```glow
set telemetry to {
  endpoint:          "/v1/chat/completions",
  raw_tokens:        raw_tokens,
  retained_fraction: f_star,
  density_score:     p_structural
}

set response to post "http://127.0.0.1:8010/telemetry" with telemetry
```

---

## What is not yet claimed

- Daemon-grade recurring background scheduler
- Broad MCP adapter ecosystem beyond filesystem
- Full production hardening across every operational edge case
- Equivalence to a general-purpose systems language runtime

The runtime is serious and executable. The claims are intentionally bounded.
