# GlowScript Language Guide

GlowScript is a scripting language for AI automation. GlowFlow is the runtime and product surface around it.

The core design bet: automation scripts should read like ordered instructions, not like framework configuration or application code.

---

## Design principles

**Named operations over punctuation.** `set name to "Ana"` instead of `name = "Ana"`. `for each lead in leads` instead of `for (let l of leads)`. The goal is that a non-developer can read a GlowScript file and understand what it does.

**AI as first-class syntax.** `extract`, `classify`, `decide`, `summarize`, and `rewrite` are language keywords, not library calls. This is the most distinctive part of the language.

**Workflow primitives built in.** Retries, approvals, webhooks, scheduling, and MCP tool exports are native, not bolted on through external SDKs.

**Small, honest value model.** Strings, numbers, booleans, lists, and objects. No type annotations, no class hierarchies, no generics. The language stays readable at the cost of being less general.

---

## Value model

### Primitive values

```glow
set name    to "Ana"
set score   to 95
set enabled to true
set missing to null
```

### Structured values

```glow
set tags to ["vip", "urgent"]
set lead to {name: "Ana", company: "Glow", score: 95}
set email to lead.email
```

### The `result` variable

Most AI and structured operations write output into `result` when no explicit variable is named. Always capture `result` promptly in longer scripts to avoid confusion.

```glow
classify message as:
  urgent
  normal
  spam

set priority to result
```

### HTTP response shape

```glow
set response to get "https://api.example.com/status"

say response.status       # 200
say response.body         # parsed JSON or raw string
say response.status_text  # "OK"
```

---

## Variables

```glow
set name to "Ana"           # mutable variable
const api_url to "https://api.example.com"  # fixed after definition
```

---

## Output

```glow
say "Automation started"
log {status: "running", count: 42}
reply "Webhook response body"
```

`say` is for human-readable output. `log` is for diagnostic/structured output. `reply` is for webhook response bodies.

---

## Control flow

```glow
if score > 80
  say "Great score"
else
  say "Needs improvement"

repeat 3 times
  say "Working"

for each lead in leads
  say lead.email

retry 3 times
  get "https://api.example.com/orders"

parallel
  run task sync_sales
  run task sync_inventory

wait 5 seconds
```

Note: `parallel` expresses grouped execution intent. It is an orchestration primitive in the current runtime, not a full concurrent scheduling guarantee.

---

## Tasks, functions, and tools

**Tasks** are automation steps — named blocks you define once and run by name.

```glow
task sync_sales
  set data to get "https://api.shop.com/sales"
  save json data to "sales.json"

run task sync_sales
```

**Functions** are reusable logic units with named parameters and a return value.

```glow
function greet
  needs name
  return "Hello {name}"

say call function greet with {name: "Ana"}
```

**Tools** are MCP-compatible callable units that can be exported to AI models.

```glow
tool create_invoice
  needs name, total
  return {name: name, total: total}

export mcp tool create_invoice

set invoice to call tool "create_invoice" with {name: "Ana", total: 1200}
```

---

## Imports

```glow
import "shared.glow"
```

Imports resolve relative to the importing file. Imported files can contribute functions, tools, and constants.

---

## Webhooks and scheduling

```glow
when webhook "/signup"
  read json body
  extract from body.message:
    name
    email
  reply "Received from {body.email}"

every 1 hour
  run task sync_sales
```

Run with `glow serve file.glow 3000`. HTTPS serving is available with `--https --cert --key`.

---

## File, JSON, CSV, and HTTP

These are native language operations, not library imports.

```glow
set text    to read file "report.txt"
set config  to load json "config.json"
set records to load csv "leads.csv"

save result     to "output.txt"
save json lead  to "lead.json"
save csv rows   to "export.csv"

set response to get "https://api.example.com/data"
set result   to post "https://api.example.com/lead" with {name: "Ana"}
set result   to put "https://api.example.com/lead/1" with {status: "active"}
set result   to delete "https://api.example.com/lead/1"
```

---

## AI as language syntax

GlowScript treats AI operations as native language primitives.

**Prompt-style**
```glow
ask ai "Summarize this report: {text}"

ask ai
  provider "openai"
  model "gpt-5"
  system "You summarize business reports concisely"
  user "Summarize: {text}"
  retries 2
```

**Structured AI forms**
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

rewrite reply in warm professional tone

decide next_step from context:
  send_invoice
  ask_for_details
  escalate_to_human
```

All structured forms accept an optional `using ai:` block for provider and model configuration.

Supported providers: `openai`, `ollama`, `command`, `mock`.

---

## MCP integration

```glow
use mcp filesystem

set text to call tool "filesystem.read_file" with {path: "notes.txt"}
set tools to list tools

tool create_invoice
  needs name, total
  return {name: name, total: total}

export mcp tool create_invoice
```

---

## Error handling

```glow
try
  set data to read file "config.json"
catch error
  set data to {status: "fallback"}
  log "File missing: {error}"

throw "Missing required field: email"

# recover is a postfix expression-level fallback
set response to get "https://api.example.com" recover {status: "offline"}
set text      to read file "notes.txt"        recover "No notes found"
```

---

## Pipeline

```glow
read file "report.txt"
pipe summarize
pipe save "summary.txt"

read file "report.txt"
pipe function clean_text
pipe say
```

---

## Context control

This is the most distinctive part of the language surface beyond basic AI primitives. GlowScript can estimate token density, compute a safe retention fraction, and compress text — all as language-level operations.

```glow
set raw_tokens    to calculate tokens task_content
set p_structural  to jerina probability raw_tokens with 1.618
set f_star        to optimal allocation p_structural with 3.0
set optimized     to collapse task_content to f_star

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
  require:
    summary
```

The `compress` block is a language-level compression policy — not a summarization call. It operates deterministically on structure, not semantically. The `auto target` path derives a safe retention fraction from the input density automatically.

---

## Agents

```glow
build agent support_bot
  on webhook "/support"
  use mcp filesystem
  memory session
  system "You are a helpful support assistant"
  user body.message
  reply result
```

`build agent` is a compact, opinionated shortcut for the common webhook-based AI agent pattern. It is not a fully open-ended agent framework — that is intentional.

---

## Memory and workflow

```glow
remember user_name     # persist value to runtime session memory

approve "Send invoice to {customer}?"
say result             # "yes" or "no"
```

---

## What GlowScript is not

GlowScript is not trying to be Python, JavaScript, or a general-purpose systems language. It does not have:

- Type annotations
- Classes or object methods
- A package manager or dependency system
- Async/await syntax (use `parallel` and `retry` instead)

These are intentional omissions. The language stays readable and scoped by not trying to do everything.
