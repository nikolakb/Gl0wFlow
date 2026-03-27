# GlowFlow

**Plain-English AI automation powered by GlowScript.**

GlowFlow is the runtime and product surface for GlowScript — a scripting language built for one job: writing AI automations that stay readable.

> Current version: **GlowFlow v1.1.2 / GlowScript v1.1.2**

---

## What it is

GlowFlow lets you build agents, webhook handlers, workflow automations, API integrations, MCP tools, and business process scripts in a language that reads like ordered instructions — not framework boilerplate.

Instead of scattering logic across Python glue code, shell scripts, YAML workflows, and prompt fragments, GlowScript puts everything in one readable surface.

```glow
when webhook "/signup"
  read json body

  extract from body.message:
    name
    email
    company

  classify body.message as:
    urgent
    normal
    spam

  decide next_step from body.message:
    send_welcome_email
    ask_for_details
    escalate_to_human

  reply "Signup received for {body.email}"
```

---

## Why it exists

Python is flexible but makes simple automations feel lower-level than they need to be. Shell scripts are fragile. YAML workflows fall apart when logic gets complex.

GlowFlow sits in the gap:

- easier to read than Python automation code
- safer and more structured than shell scripts
- more expressive for AI workflows than YAML-heavy tools
- natively MCP-compatible without extra wiring

---

## Get started in 5 minutes

**Requirements:** Rust toolchain installed (`rustup.rs`)

```bash
# Clone and build
git clone https://github.com/nikolakb/Gl0wFlow.git
cd Gl0wFlow
cargo build --release

# Run your first script
cargo run -- run examples/hello.glow

# Run the investor demo
cargo run -- run examples/investor_demo.glow

# Start a webhook server
cargo run -- serve examples/webhook_signup.glow 3000
```

Or install via script:

```bash
./scripts/install-from-repo.sh https://github.com/nikolakb/Gl0wFlow.git
```

---

## CLI commands

| Command | What it does |
|---|---|
| `glow run file.glow` | Execute a script |
| `glow build file.glow` | Compile to native binary via Rust |
| `glow check file.glow` | Semantic analysis and diagnostics |
| `glow format file.glow` | Format and normalize a script |
| `glow inspect file.glow` | Print the AST |
| `glow serve file.glow [port]` | Run a webhook server (HTTP or HTTPS) |
| `glow repl` | Interactive REPL |
| `glow new project` | Scaffold a new project |

---

## Language in 60 seconds

**Variables and output**
```glow
set name to "Ana"
say "Hello {name}"
```

**Control flow**
```glow
if score >= 80
  say "Great"
else
  say "Try again"

repeat 3 times
  say "Working"

for each lead in leads
  say lead.email
```

**Tasks and functions**
```glow
task sync_sales
  get "https://api.shop.com/sales"
  save result to "sales.json"

run task sync_sales
```

**AI as native syntax**
```glow
extract from message:
  name
  email
  company

classify message as:
  urgent
  normal
  spam

decide next_step from context:
  send_invoice
  ask_for_details
  escalate_to_human

summarize report into short_summary
rewrite reply in friendly tone
```

**Files, JSON, HTTP**
```glow
set data to load json "leads.json"
set response to get "https://api.example.com/status"
save json result to "output.json"
```

**Error handling**
```glow
try
  set data to read file "config.json"
catch error
  set data to {status: "fallback"}

set response to get "http://api.example.com" recover {status: "offline"}
```

**MCP tools**
```glow
use mcp filesystem

tool create_invoice
  needs name, total
  return {name: name, total: total}

export mcp tool create_invoice
```

**Context compression**
```glow
compress report into brief
  auto target
  w_j 1.618
  gain 3.0
  mode agent-safe
  preserve:
    summary
    action items
  keep:
    first sentence
    last sentence
```

**Agents**
```glow
build agent support_bot
  on webhook "/support"
  use mcp filesystem
  memory session
  system "You are a helpful support assistant"
  user body.message
  reply result
```

---

## What is already working

- Full compiler pipeline: lexer → parser → AST → semantic analyzer → interpreter → Rust transpiler
- Native binary compilation via `glow build` when `rustc` is present
- HTTP and HTTPS webhook serving
- File, JSON, CSV operations
- AI execution via `openai`, `ollama`, `command`, and `mock` providers
- MCP filesystem tools and user-defined tool exports
- First-class `try` / `catch` / `throw` / `recover`
- Context-control built-ins: `calculate tokens`, `compress`, `collapse`
- Formatter, AST inspector, REPL, project scaffolding
- 160+ tests across the full language surface (consolidated to focused regression suite in v1.1.x)

---

## What is not yet claimed

- A full production orchestration platform
- A background daemon scheduler
- A broad MCP adapter ecosystem beyond filesystem
- Complete production hardening across every edge case

---

## Architecture

```
GlowScript source
  → Lexer
  → Parser
  → AST
  → Semantic Analyzer
  → Interpreter (glow run)
     or
  → Rust Transpiler → rustc → native binary (glow build)
```

The parser is intentionally small and readable. Diagnostics carry line and column information with suggested fixes.

---

## Performance targets

| Operation | Target |
|---|---|
| CLI startup, small script | < 100ms |
| Parse + semantic check | < 50ms |
| Interpreter, simple logic | < 100ms |
| Webhook response, no AI | 50–300ms |
| Full local automation | < 1s |

AI provider calls, HTTP APIs, and external tools will dominate real-world latency. GlowFlow aims to add near-zero overhead on top of the work being orchestrated.

---

## License

Apache-2.0
