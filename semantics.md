# GlowScript Semantics and Capabilities

GlowFlow is the product, runtime, and documentation surface. GlowScript is the language.

This document explains GlowScript as a language rather than as a product pitch. It covers:

- the execution model
- the value model
- statement semantics
- control flow
- AI primitives
- tool orchestration
- webhooks and HTTP behavior
- error handling
- semantic checks
- current capabilities and boundaries

Version: `GlowFlow v1.1.2` / `GlowScript v1.1.2`

## 1. Core Model

GlowScript is a statement-first scripting language for AI automation.

It is designed around these ideas:

- automation should read like ordered instructions
- the language should prefer named operations over punctuation-heavy syntax
- common workflow tasks should be native language features instead of library calls
- AI operations should be part of the language surface, not bolted on through helper code

GlowScript is indentation-based. Blocks are introduced by statements such as `if`, `repeat`, `task`, `when webhook`, `try`, and similar forms. The visual structure of the script defines the execution structure.

Example:

```glow
set name to "Ana"

if name == "Ana"
  say "Hello Ana"
else
  say "Hello"
```

## 2. Execution Model

GlowScript currently has two main execution paths:

1. `glow run`
   Executes a script through the interpreter runtime.

2. `glow build`
   Produces a native runner path that preserves the current supported language semantics.

The interpreter is the semantic reference path. The native build path exists so GlowScript programs can be deployed as native executables without changing the workflow language itself.

## 3. Value Model

GlowScript supports a deliberately small value system.

### Primitive values

- strings
- numbers
- booleans
- `null`

Examples:

```glow
set name to "Ana"
set score to 95
set enabled to true
set missing to null
```

### Structured values

- lists
- objects

Examples:

```glow
set tags to ["vip", "urgent"]
set lead to {name: "Ana", company: "Glow", score: 95}
```

### Runtime-shaped values

Some built-ins return object-shaped values with a known runtime structure.

HTTP responses are represented as objects with:

- `status`
- `status_text`
- `headers`
- `body`
- `text`

Example:

```glow
set response to get "http://127.0.0.1:8080/health"

if response.status == 200
  say response.body
```

### The `result` value

Many operations write their output into `result` when they do not write into an explicitly named variable.

This is especially common with:

- `ask ai`
- `extract`
- `classify`
- `rewrite`
- pipeline stages

Example:

```glow
ask ai "Summarize this report"
save result to "summary.txt"
```

## 4. Variables and Assignment

GlowScript uses explicit assignment keywords.

- `set <name> to <expr>`
- `const <name> to <expr>`

Examples:

```glow
set name to "Ana"
const api_url to "https://api.shop.com"
```

`set` introduces or updates a variable.

`const` introduces a value intended to be fixed after definition. The current language surface treats constants as distinct declarations for clarity and semantics.

## 5. Statements

GlowScript is primarily built from named statements.

### Output and diagnostics

- `say <expr>`
- `log <expr>`

Examples:

```glow
say "Automation started"
log {status: "running"}
```

### File and data output

- `save <expr> to <expr>`
- `save json <expr> to <expr>`
- `save csv <expr> to <expr>`

Examples:

```glow
save result to "summary.txt"
save json lead to "lead.json"
save csv rows to "rows.csv"
```

### Return and reply

- `return <expr>`
- `reply <expr>`

`return` is used in functions and tools.

`reply` is used in webhook/server flows.

## 6. Expressions

Expressions are the pieces of code that produce values.

Supported expression categories include:

- strings
- numbers
- booleans
- identifiers
- property access
- lists
- objects
- file/data loading
- HTTP calls
- environment variable reads
- built-in runtime functions
- function calls
- tool calls
- binary operators
- postfix `recover`

### Property access

Objects are traversed using dot access.

```glow
set email to body.email
```

### File and data loading

- `read file "notes.txt"`
- `load json "data.json"`
- `load csv "leads.csv"`

### HTTP expressions

- `get "http://..."`
- `post "http://..." with {...}`
- `put "http://..." with {...}`
- `delete "http://..."`

### Environment and built-ins

- `env "API_KEY"`
- `now`
- `random`
- `length value`
- `calculate tokens value`
- `jerina probability density with w_j`
- `optimal allocation p with b`
- `collapse text to fraction`
- `list tools`

### Function and tool calls

- `call function greet with {name: "Ana"}`
- `call tool "filesystem.read_file" with {path: "notes.txt"}`

### Operators

GlowScript currently supports:

- `+`
- `==`
- `!=`
- `>`
- `>=`
- `<`
- `<=`

Example:

```glow
if score >= 80
  say "Great"
```

## 7. Control Flow

### `if` / `else`

Conditional branching is explicit and indentation-based.

```glow
if score > 80
  say "Great"
else
  say "Try again"
```

Conditions evaluate on truthy values, but semantic analysis also rejects some clearly invalid condition shapes, such as object-like values used directly in condition position.

### `repeat`

`repeat` loops a numeric number of times.

```glow
repeat 3 times
  say "Jump"
```

### `for each`

`for each` iterates over lists.

```glow
for each lead in leads
  say lead.email
```

Semantic analysis expects the iterated value to be list-compatible.

## 8. Workflow Units

GlowScript separates different kinds of reusable behavior.

### Tasks

Tasks are reusable automation steps.

```glow
task sync_sales
  get "https://api.shop.com/sales"
```

Tasks are invoked with:

```glow
run task sync_sales
```

### Build agents

GlowScript also supports high-level agent declarations.

```glow
build agent support_bot
  on webhook "/support"
  use mcp filesystem
  memory session
  system "You are a helpful support assistant"
  user body.message
  reply result
```

Current `build agent` semantics:

- the trigger is currently `on webhook "..."`
- `use mcp` activates tool targets for the agent runtime
- `memory session` loads and stores lightweight session memory
- `system`, `provider`, `model`, and `retries` configure the AI call
- `user <expr>` becomes the prompt input
- `reply <expr>` becomes the webhook response

In the current runtime, `build agent` is intentionally compact and opinionated. It is a language-level shortcut for the common “AI webhook agent” shape, not a fully open-ended agent framework.

### Functions

Functions are reusable logic units with named parameters.

```glow
function greet
  needs name
  return "Hello {name}"
```

Called with:

```glow
say call function greet with {name: "Ana"}
```

### Tools

Tools are reusable tool-shaped capabilities and can also be exported through the MCP surface.

```glow
tool create_invoice
  needs name, total
  return {name: name, total: total}
```

Called with:

```glow
set invoice to call tool "create_invoice" with {name: "Ana", total: 14}
```

Exported with:

```glow
export mcp tool create_invoice
```

## 9. Imports

GlowScript supports file-level imports for reusable shared logic.

```glow
import "shared.glow"
```

Imported files can contribute functions, tools, constants, and other reusable declarations.

Import resolution is relative to the importing script path.

## 10. Scheduling and Events

### Schedules

GlowScript supports schedule-shaped syntax:

```glow
every 1 hour
  run task sync_sales
```

This is currently part of the language surface and runtime model. It expresses schedule intent clearly, though the project does not yet claim a full daemon-style background scheduler as a hardened production subsystem.

### Webhooks

GlowScript supports event entry points through webhook handlers.

```glow
when webhook "/signup"
  read json body
  reply "Signup received"
```

Inside webhook handlers:

- `read json body` populates `body`
- `reply <expr>` returns the response body

`glow serve` can now run both HTTP and HTTPS webhook servers.

## 11. Files, JSON, CSV, and HTTP

These are native language operations, not library calls.

### Files

- `read file`
- `save`
- `append file` is not a first-class form in the current documented surface

### JSON

- `load json`
- `save json`

### CSV

- `load csv`
- `save csv`

### HTTP

- `get`
- `post`
- `put`
- `delete`

Example:

```glow
set lead to {name: "Ana", company: "Glow"}
set response to post "https://crm.example.com/lead" with lead

if response.status == 200
  say "Lead sent"
```

## 12. AI as Language Surface

GlowScript treats AI as native syntax.

### Prompt-style AI

```glow
ask ai "Summarize this report"
```

or block form:

```glow
ask ai
  provider "ollama"
  model "qwen3-coder:480b-cloud"
  system "You summarize business reports"
  user "Summarize this report"
```

### Structured AI forms

- `extract`
- `classify`
- `summarize`
- `rewrite`
- `decide`

Examples:

```glow
extract from message:
  name
  email
  company
```

```glow
classify interest as:
  high
  medium
  low
```

```glow
summarize report into short_summary
```

```glow
rewrite reply in friendly tone
```

```glow
decide next_step from context:
  send_invoice
  ask_for_details
  escalate_to_human
```

### AI config block entries

Structured AI blocks can include configuration such as:

- `provider "openai"`
- `provider "ollama"`
- `provider "command"`
- `provider "mock"`
- `model "gpt-5"`
- `system "You are ..."`
- `retries 2`
- `fields:`
- `labels:`
- `options:`

## 13. MCP and Tool Orchestration

GlowScript has an MCP-oriented integration surface.

### Declaring usage

```glow
use mcp filesystem
```

### Calling tools

```glow
set text to call tool "filesystem.read_file" with {path: "notes.txt"}
```

### Listing tools

```glow
set tools to list tools
```

### Exporting GlowScript tools

```glow
export mcp tool create_invoice
```

The current ecosystem surface is intentionally small and credible. GlowScript does not yet claim a fully broad MCP adapter ecosystem.

## 14. Pipeline Semantics

GlowScript includes a `pipe` mechanism for readable chaining.

Examples:

```glow
read file "report.txt"
pipe summarize
pipe save "summary.txt"
```

```glow
read file "report.txt"
pipe function clean_text
pipe say
```

Current supported pipeline stages include:

- summarize
- rewrite
- save
- say
- log
- function calls
- tool calls

The current model is a practical stage-based pipeline for workflow readability. It is not yet a fully general dataflow calculus.

## 15. Memory and Human Workflow Features

GlowScript includes workflow-oriented built-ins beyond core control flow.

### `remember`

```glow
remember user_name
```

This persists a created value into runtime memory storage.

### `approve`

```glow
approve "Send invoice to customer?"
```

This is intended as a human checkpoint in business workflows.

### `wait`

```glow
wait 5 seconds
```

The current surface supports duration-style waiting.

### `retry`

```glow
retry 3 times
  get "https://api.shop.com/orders"
```

This expresses operational retry behavior directly in the language.

### `parallel`

```glow
parallel
  run task sync_sales
  run task sync_inventory
```

In the current `v1.1.2` runtime, `parallel` is grouped execution intent. It should be read as an orchestration primitive, not as a claim of fully general concurrent scheduling semantics.

## 16. Error Handling Semantics

`v1.1.2` includes first-class error recovery.

### `throw`

```glow
throw "Missing email"
```

This raises a runtime error using the rendered text form of the value.

### `try` / `catch`

```glow
try
  set data to read file "missing.json"
catch error
  set data to "{status: \"fallback\"}"
```

The `catch` variable receives the runtime error message text.

### `recover`

```glow
set status to get "http://127.0.0.1:9/health" recover {status: "offline"}
```

`recover` is a postfix expression form. It returns the fallback only when the wrapped expression fails.

This is especially useful around:

- file reads
- HTTP calls
- tool calls
- other expression-level operations that may fail

## 17. Semantic Analysis

GlowScript performs semantic checks before runtime.

Examples of things checked early:

- use of undefined values
- missing tasks
- missing functions
- missing tools
- invalid object property assumptions
- numeric comparison misuse
- invalid `repeat` counts
- invalid `for each` iteration targets
- invalid `remember` usage

The goal is to produce beginner-readable diagnostics where possible, rather than low-level parser or runtime failures.

## 18. Context Control Semantics

GlowScript `v1.1.2` includes a small advanced context-control surface in the runtime.

### `calculate tokens`

```glow
set raw_tokens to calculate tokens task_content
```

This returns a numeric token estimate based on whitespace-delimited segments. It is intentionally simple and deterministic.

### `jerina probability`

```glow
set p_structural to jerina probability raw_tokens with 1.618
```

This computes an advanced density score using the runtime context-control engine.

### `optimal allocation`

```glow
set f_star to optimal allocation p_structural with 3.0
```

This computes a safe retention fraction and clamps it into the runtime range.

### `collapse`

```glow
set optimized_content to collapse task_content to f_star
```

Current runtime behavior:
- preserves paragraph boundaries when possible
- keeps whole sentences instead of cutting at raw word boundaries
- guarantees at least the lead sentence of each retained paragraph
- fills the remaining token budget deterministically in source order

This is still a deterministic structural compressor, not semantic summarization.

### `compress`

```glow
compress report into brief
  auto target
  w_j 1.618
  gain 3.0
  mode semantic
  preserve:
    summary
    action items
  keep:
    first sentence
    last sentence
  require:
    summary
```

Current runtime behavior:
- `compress` is a first-class statement, not an expression helper
- it writes the compressed result into the named output variable
- it also updates `result`
- `target` is a retention fraction between `0.05` and `1.0`
- `auto target` derives the retention fraction from the input text using the runtime context-control engine
- `w_j` and `gain` are advanced tuning inputs for that automatic path
- `mode` can be `structural`, `semantic`, or `agent-safe`
- `preserve` boosts sentences containing the listed phrases
- `keep` can force `first sentence`, `last sentence`, and `headings`
- `require` turns missing retained phrases into a readable runtime error
- safe auto floors prevent over-compression:
  - structural: at least `0.20`
  - semantic: at least `0.30`
  - agent-safe: at least `0.40`

This makes compression a language-level policy surface instead of just a truncation helper.

Publicly, the important contract is:
- GlowScript can reduce context safely
- GlowScript can preserve required material
- GlowScript can auto-size retention conservatively when asked

The internal scoring and allocation strategy is an implementation detail of the runtime.

### Telemetry pattern

GlowScript does not need a new telemetry keyword for this. The existing HTTP `post` form is enough.

```glow
set telemetry to {endpoint: "/v1/chat/completions", raw_tokens: raw_tokens, retained_fraction: f_star, density_score: p_structural}
set response to post "http://127.0.0.1:8010/telemetry" with telemetry
```

The repo includes a FastAPI companion service in:

- `telemetry/fastapi_proxy.py`

## 18. CLI Surface

The project currently exposes these CLI entry points:

- `glow help`
- `glow run file.glow`
- `glow build file.glow`
- `glow check file.glow`
- `glow format file.glow`
- `glow inspect file.glow`
- `glow repl`
- `glow new project`
- `glow serve file.glow [port]`

HTTPS serving is available with certificate and key configuration in the current release line.

## 19. Current Capabilities

GlowScript can currently do all of the following in one language surface:

- variables and constants
- branching and loops
- tasks, functions, tools, and imports
- file operations
- JSON and CSV handling
- HTTP calls
- HTTP and HTTPS webhook serving
- structured AI operations
- retries, waits, approvals, and memory
- MCP-style filesystem tool orchestration
- runtime interpretation
- native runner builds

## 20. Current Boundaries

GlowScript `v1.1.2` is serious and executable, but its claims are intentionally bounded.

It does **not** currently claim:

- a broad package ecosystem like Python or JavaScript
- a fully mature production orchestration platform
- a full MCP adapter ecosystem
- a daemon-grade recurring background scheduler
- complete production hardening across every operational edge case
- equivalence to a general-purpose systems or application language

## 21. Positioning

The cleanest positioning is:

> GlowScript is a scripting language for AI automation.

GlowFlow is the product/runtime layer around that language.

The value of the system is that files, APIs, AI actions, tools, webhooks, approvals, and workflow decisions can be expressed in one readable language instead of being scattered across shell scripts, Python glue, YAML workflows, and prompt fragments.

## 22. Minimal End-to-End Example

```glow
use mcp filesystem

set report to read file "report.txt"

extract from report:
  company
  summary

classify report as:
  urgent
  normal
  spam

decide next_step from report:
  send_invoice
  ask_for_details
  escalate_to_human

if result == "urgent"
  log "High priority workflow"

ask ai "Summarize this report"
save result to "summary.txt"
```

This example is not a full production business process by itself. It shows the core language design: readable automation steps, native AI operations, and direct workflow semantics.
