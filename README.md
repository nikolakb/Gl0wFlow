# GlowFlow

GlowFlow is the product and runtime surface for GlowScript, the scripting language for AI automation.

GlowFlow lets you build and present agents, workflows, webhook handlers, approvals, tool calls, and AI automations written in GlowScript.

Current version:
- GlowFlow v1.0.1
- GlowScript v1.0.1
- first public fix release with HTTPS webhooks and language-level error recovery

Tagline:
GlowFlow - Plain-English AI automation powered by GlowScript.


## What It Is

GlowFlow is not trying to replace general-purpose languages.

GlowFlow is built for one job:
- writing AI automations that stay readable

GlowFlow is the clearest path from idea to agent behavior.
Instead of forcing AI automation into Python boilerplate, shell fragility, or YAML sprawl, GlowFlow puts GlowScript at the center and turns the workflow itself into the language.

GlowFlow is for:
- AI agents
- workflow orchestration
- webhook automations
- API integrations
- MCP tools and tool-calling systems
- business process automation

## Why It Exists

Python and JavaScript are flexible, but they make many automations feel lower-level than they need to be.

Shell scripts are fast, but fragile.

YAML workflows are easy to start, but hard to scale into logic-heavy agent behavior.

GlowFlow sits in the gap:
- easier to read than Python automation code
- safer and more structured than shell scripts
- more direct for AI workflows than YAML-heavy tools

## What GlowScript Looks Like

```glow
when webhook "/signup"
  read json body

  extract from body.email:
    company

  decide next_step from body.email:
    send_welcome_email
    ask_for_details
    escalate_to_human

  reply "Signup received for {body.email}"
```

Or:

```glow
read file "report.txt"
pipe summarize
pipe save "summary.txt"
```

## Product Direction

GlowFlow aims for:

- Python-level simplicity
- Lua-level readability
- Rust-level deployment model

Current MVP status:

- Plain-English syntax with indentation-based blocks
- Lexer, parser, AST, diagnostics, semantic checks
- Formatter
- Interpreter for the core automation subset
- Local webhook server runtime
- HTTPS webhook server runtime
- Rust transpiler for the core subset
- CLI commands: `run`, `build`, `check`, `format`, `inspect`, `repl`, `new`, `serve`

Version framing:

- `v1.0.1` is the first fix release on top of the public `v1.0.0` line
- it adds HTTPS webhook serving and first-class `try` / `catch` / `throw` / `recover`
- it still does not claim full production hardening yet

Test status:

- 9 passing tests
- 1 focused internal regression test protects a real webhook parsing edge case
- 1 mother end-to-end test proves `check`, `inspect`, `run`, `build`, native parity, imports, files, JSON, CSV, tools, MCP filesystem access, and workflow artifact creation on a single serious scenario
- 7 focused `v1.0.1` tests cover error handling, formatter round-trip, HTTPS TLS setup, and new examples
- recent stress passes also covered same-file formatter/build churn, webhook mutation, deep import chains, large JSON/CSV payloads, and repeated native parity
- local Ollama-backed proof also covered real provider runs for `extract`, `classify`, `decide`, and `summarize`
- the current proof surface is intentionally compact and centered on one full-language scenario plus focused regressions
- a human-readable test proof summary lives at [docs/tests.html](./docs/tests.html)

The current runtime already supports real local execution for:

- files
- JSON and CSV loading/saving
- plain HTTP `get`/`post`/`put`/`delete` over `http://`
- webhook serving on `127.0.0.1`
- HTTPS webhook serving with `--https --cert --key`
- per-request webhook persistence with unique saved payload files
- environment-based and provider-based AI execution
- MCP-style filesystem tools and user-defined Glow tools

## Performance Targets

GlowFlow is designed so the language overhead stays near-instant and most real
latency comes from the work being orchestrated, not from GlowScript itself.

For server-side MVP use, the practical targets are:

- CLI startup for small scripts: under `100ms`
- parse plus semantic check for a typical script: under `50ms`
- local interpreter execution for simple logic: under `100ms`
- simple webhook response without external AI or API calls: `50ms` to `300ms`
- normal automation runs with local work only: under `1s`

The important distinction is that AI calls, HTTP APIs, file I/O, and external
tools will usually dominate total runtime. GlowFlow should add as little extra
latency on top of that as possible.

## Quick Start

```bash
cargo run -- run examples/investor_demo.glow
```

Or run the showcase:

```bash
./scripts/investor-demo.sh
```

## Syntax Rules

Glow uses:

- one statement per line
- indentation with two spaces per block level
- comments starting with `#`
- plain-English statements instead of punctuation-heavy syntax

Language backbone:

- `set`
- `say`
- `if`
- `else`
- `repeat`
- `for each`
- `task`
- `when`
- `every`
- `ask`
- `use`
- `tool`

Example:

```glow
say "Automation started"
set name to "Ana"

if name == "Ana"
  say "Hello Ana"

repeat 3 times
  say "Working"
```

Core rules:

1. Variable assignment uses `set name to value` or `const api_key to value`.
2. Conditions use `if ...` with optional `else`.
3. Loops use `repeat n times` and `for each item in items`.
4. Tasks use `task name` followed by an indented block.
5. Task execution uses `run task name`.
6. Webhooks use `when webhook "/path"`.
7. Schedules use `every 1 hour`.
8. AI calls use `ask ai "prompt"` or an indented AI config block.
9. MCP imports use `use mcp filesystem`.

## MVP feature set

Implemented now:

- literals: strings, numbers, booleans
- variables and constants
- lists and object literals
- property access: `body.email`
- `say`, `log`
- `set`, `const`
- `if`, `else`
- `repeat`, `retry`, `for each`, `parallel`
- `task`, `run task`
- `function`, `call function`
- `import "other.glow"`
- `wait`
- `save value to "file.txt"`
- `save json`, `load json`
- `save csv`, `load csv`
- `get`, `post`, `put`, `delete`
- `reply`, `approve`
- `remember`
- `try`, `catch`, `throw`, `recover`
- `pipe`
- `read json body`
- `tool`, `return`, `export mcp tool`
- `call tool`, `list tools`
- `env`, `now`, `random`, `length`
- expressions: `+`, `==`, `!=`, `>`, `>=`, `<`, `<=`
- beginner-friendly parse and semantic diagnostics
- formatter and AST inspection

Executed with current runtime boundaries:

- `every` registers the schedule and runs the body immediately in MVP mode
- `when webhook` runs through `glow serve`
- `ask ai` and structured AI forms honor `provider`, `model`, `system`, and `retries`
- supported MVP AI providers are `command`, `mock`, `openai`, and `ollama`
- if no provider is selected, Glow uses `GLOW_AI_COMMAND` when present and otherwise returns a stub
- `use mcp filesystem` exposes filesystem-style tool calls

Planned next:

- richer HTTP client options and more production network controls
- native scheduler daemon mode
- broader MCP adapters beyond filesystem

## Compiler architecture

Glow source flows through:

1. Lexer
2. Parser
3. AST
4. Semantic analyzer
5. Interpreter or Rust transpiler
6. Native compilation through Rust toolchain

Architecture notes:

- The parser is intentionally small and readable.
- Diagnostics carry line and column information with suggested fixes.
- The interpreter makes `glow run` usable before native compilation is available.
- The transpiler generates standalone Rust source from the AST so the project can evolve toward native binaries cleanly.

## Three example Glow programs

Hello world:

```glow
say "Hello from Glow"
```

AI summarizer:

```glow
set text to read file "report.txt"
ask ai "Summarize this text: {text}"
save result to "summary.txt"
```

Workflow example:

```glow
task sync_sales
  say "Syncing sales"
  save "done" to "sales.log"

every 1 hour
  run task sync_sales
```

Tool export example:

```glow
tool create_invoice
  needs name, total
  return {name: name, total: total}

export mcp tool create_invoice

set invoice to call tool "create_invoice" with {name: "Ana", total: 1200}
say invoice
```

Function example:

```glow
function greet
  needs name
  return "Hello {name}"

say call function greet with {name: "Ana"}
```

Structured AI example:

```glow
extract from message:
  name
  email
  company

classify message as:
  urgent
  normal
  spam

summarize report into short_summary
rewrite reply in friendly tone

decide next_step from context:
  send_invoice
  ask_for_details
  escalate_to_human
```

Minimal automation example:

```glow
use mcp filesystem

set report to read file "report.txt"
ask ai "Summarize this report"
save result to "summary.txt"

repeat 2 times
  say "Done"
```

Workflow control example:

```glow
set customer to "Ana"
remember customer

retry 2 times
  log "sync started"

parallel
  say "sales sync"
  say "inventory sync"

approve "Send invoice to {customer}?"
say result

wait 0 seconds
```

Pipe example:

```glow
read file "report.txt"
pipe summarize
pipe save "summary.txt"
pipe say
```

Configured AI block example:

```glow
extract from message using ai:
  provider "openai"
  model "gpt-5"
  system "Extract customer details"
  fields:
    name
    email
```

HTTP response example:

```glow
set response to get "http://127.0.0.1:3000/health"

if response.status == 200
  say response.body
```

Investor demo:

```glow
say "Glow demo: one inbound message becomes an action plan"

set inbound_message to "Name: Ana email ana@example.com company Glow. Urgent request. Missing shipping address. Customer asked for pricing."

extract from inbound_message:
  name
  email
  company

set lead to result

classify inbound_message as:
  urgent
  normal
  spam

set priority to result

decide next_step from inbound_message:
  ask_for_details
  send_invoice
  escalate_to_human

set next_step to result

rewrite "Thanks for reaching out. We can help right away." in friendly tone
set draft_reply to result

say "Lead: {lead.name} <{lead.email}> from {lead.company}"
say "Priority: {priority}"
say "Next step: {next_step}"
say "Draft reply: {draft_reply}"
```

Run it with:

```bash
cargo run -- run examples/investor_demo.glow
```

Or use the one-command showcase:

```bash
./scripts/investor-demo.sh
```

## Development phases

Phase 1:

- language syntax
- grammar
- AST
- diagnostics

Phase 2:

- parser
- formatter
- CLI
- interpreter

Phase 3:

- automation primitives
- task orchestration
- file and HTTP helpers

Phase 4:

- AI features
- retries
- structured outputs

Phase 5:

- MCP integrations
- MCP exports

Phase 6:

- optimization
- packaging
- richer documentation

## Build and run

If Rust is available:

```bash
cargo run -- run examples/hello.glow
cargo run -- serve examples/webhook_signup.glow 3000
```

`glow build` now emits a native binary when `rustc` is installed. If not, it falls back to the generated `.rs` file.
