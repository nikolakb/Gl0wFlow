# GlowFlow Language Notes

GlowScript is designed as a small scripting language for AI automation, and GlowFlow is the product/runtime surface around it.

The goal is to make agents, workflows, approvals, tool calls, and business automations read like instructions instead of framework code.

## Language backbone

GlowScript's parser is intentionally built around a very small set of backbone words:

- `set` assigns values
- `say` prints values
- `if` and `else` control decisions
- `repeat` and `for each` handle loops
- `task` defines automation steps
- `when` handles event triggers
- `every` handles schedules
- `ask` invokes AI
- `use` connects integrations such as MCP targets
- `tool` defines reusable tool-shaped functions
- `build agent` defines a high-level agent entry point

That keeps the language readable while still covering most automation scripts.

## Plain-English grammar

Statements:

- `say <expr>`
- `log <expr>`
- `set <name> to <expr>`
- `const <name> to <expr>`
- `save <expr> to <expr>`
- `save json <expr> to <expr>`
- `save csv <expr> to <expr>`
- `if <expr>` followed by indented block
- `else` followed by indented block
- `repeat <expr> times` followed by indented block
- `for each <name> in <expr>` followed by indented block
- `task <name>` followed by indented block
- `function <name>` followed by optional `needs ...` and an indented block
- `run task <name>`
- `import "<path>"`
- `every <expr> <minute|hour|day>` followed by indented block
- `when webhook "<path>"` followed by indented block
- `extract from <expr>:` followed by field names
- `classify <expr> as:` followed by labels
- `summarize <expr> into <name>`
- `rewrite <expr> in <tone>`
- `decide <name> from <expr>:` followed by options
- `ask ai <expr>`
- `ask ai` followed by an AI config block
- `use mcp <name>`
- `reply <expr>`
- `read json body`
- `tool <name>` followed by `needs ...` and an indented block
- `build agent <name>` followed by:
  - `on webhook "..."`
  - optional `use mcp ...`
  - optional `memory session`
  - optional `provider`, `model`, `retries`
  - `system "..."`
  - `user <expr>`
  - `reply <expr>`
- `return <expr>`
- `export mcp tool <name>`

Expressions:

- strings: `"hello"`
- numbers: `42`, `3.14`
- booleans: `true`, `false`
- identifiers: `name`
- property access: `body.email`
- lists: `[1, 2, 3]`
- objects: `{name: "Ana", total: 14}`
- read file: `read file "notes.txt"`
- load json: `load json "data.json"`
- load csv: `load csv "leads.csv"`
- http get: `get "http://127.0.0.1:8080/data"`
- http post: `post "http://127.0.0.1:8080/data" with {name: "Ana"}`
- http put: `put "http://127.0.0.1:8080/data/1" with {status: "done"}`
- http delete: `delete "http://127.0.0.1:8080/data/1"`
- http responses are objects with `status`, `status_text`, `headers`, `body`, and `text`
- env: `env "API_KEY"`
- now, random, length
- token analysis: `calculate tokens text`
- advanced density scoring: `jerina probability density with 1.618`
- retention sizing: `optimal allocation probability with 3.0`
- text compaction: `collapse text to 0.45`
  - current runtime keeps whole sentences and paragraph shape where possible
- policy compression:
  - `compress report into brief`
  - supports `target` or `auto target`
  - auto mode accepts `w_j` and `gain`
  - supports `mode`, `preserve`, `keep`, and `require`
- call tool: `call tool "filesystem.read_file" with {path: "notes.txt"}`
- call function: `call function greet with {name: "Ana"}`
- list tools
- binary operators: `+`, `==`, `!=`, `>`, `>=`, `<`, `<=`

AI config block entries:

- `provider "openai"`
- `model "gpt-5"`
- `system "You are ..."`
- `retries 2`
- `user "Prompt text"` for block `ask ai`
- `fields:`, `labels:`, or `options:` for structured AI blocks

## Seven core built-in commands

GlowScript's smallest useful automation toolkit is:

- `read`
- `save`
- `get`
- `post`
- `log`
- `reply`
- `run`

Those commands, combined with the backbone keywords above, are enough to express most lightweight automation flows.

Minimal example:

```glow
use mcp filesystem

set report to read file "report.txt"

ask ai "Summarize this report"

save result to "summary.txt"

repeat 2 times
  say "Done"
```

Agent example:

```glow
build agent support_bot
  on webhook "/support"
  use mcp filesystem
  memory session
  system "You are a helpful support assistant"
  user body.message
  reply result
```

## MVP boundaries

GlowScript currently favors a credible compiler core over a full production runtime. The local runtime now includes file, JSON, CSV, HTTP and HTTPS webhook handling, AI-command hooks, filesystem MCP tools, first-class `try` / `catch` / `throw` / `recover`, and advanced context-control built-ins, while background scheduling and broader MCP adapters remain future work.

## Structural allocation and telemetry

GlowScript now also includes a small advanced context-control surface:

- `calculate tokens text`
- `jerina probability density with w_j`
- `optimal allocation p with b`
- `collapse text to f_star`
- `compress value into name`
  - `target <fraction>`
  - `mode structural|semantic|agent-safe`
  - `preserve:` block
  - `keep:` block with `first sentence`, `last sentence`, `headings`
  - `require:` block

Example:

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

The public contract is simple:
- GlowScript can estimate a safe retention target automatically
- GlowScript can preserve required content while reducing context size
- the internal scoring model is intentionally treated as implementation detail

This is intended for agent/runtime optimization flows where a script needs to estimate input density, pick a retention fraction, and compact text before handing it to another system.

FastAPI telemetry does not require new syntax. Use the built-in HTTP `post` with a JSON-shaped object:

```glow
set telemetry to {endpoint: "/v1/chat/completions", retained_fraction: f_star, density_score: p_structural}
set response to post "http://127.0.0.1:8010/telemetry" with telemetry
say response.status
```
