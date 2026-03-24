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

## MVP boundaries

GlowScript currently favors a credible compiler core over a full production runtime. The local runtime now includes file, JSON, CSV, HTTP and HTTPS webhook handling, AI-command hooks, filesystem MCP tools, and first-class `try` / `catch` / `throw` / `recover`, while background scheduling and broader MCP adapters remain future work.
