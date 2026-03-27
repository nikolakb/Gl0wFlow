# GlowScript Grammar Reference

GlowScript uses indentation-based blocks and plain-English statements. There are no semicolons, no curly braces, and no punctuation-heavy syntax.

---

## Syntax rules

- One statement per line
- Blocks are opened by a statement and indented with two spaces
- Comments start with `#`
- String interpolation uses `{variable}` inside double-quoted strings

---

## Core keywords

These are the backbone words the parser is built around:

| Keyword | Role |
|---|---|
| `set` | Variable assignment |
| `const` | Constant declaration |
| `say` | Print to output |
| `log` | Diagnostic output |
| `if` / `else` | Conditional branching |
| `repeat` | Count-based loop |
| `for each` | List iteration |
| `task` | Automation step definition |
| `function` | Reusable logic unit |
| `tool` | MCP-compatible tool definition |
| `when` | Event trigger (webhooks) |
| `every` | Schedule trigger |
| `ask` | AI invocation |
| `use` | Integration activation (MCP) |
| `build agent` | High-level agent definition |
| `try` / `catch` / `throw` / `recover` | Error handling |
| `pipe` | Pipeline chaining |
| `compress` | Context compression |

---

## Statement forms

### Assignment
```
set <name> to <expr>
const <name> to <expr>
```

### Output
```
say <expr>
log <expr>
reply <expr>
return <expr>
```

### File and data
```
save <expr> to <expr>
save json <expr> to <expr>
save csv <expr> to <expr>
```

### Control flow
```
if <expr>
  <block>
else
  <block>

repeat <expr> times
  <block>

for each <name> in <expr>
  <block>

retry <expr> times
  <block>

parallel
  <block>

wait <expr> seconds
```

### Definitions
```
task <name>
  <block>

function <name>
  needs <param>, <param>, ...
  <block>

tool <name>
  needs <param>, <param>, ...
  <block>
```

### Invocation
```
run task <name>
call function <name> with {<args>}
call tool "<name>" with {<args>}
```

### Import and export
```
import "<path>"
export mcp tool <name>
```

### Events
```
when webhook "<path>"
  <block>

every <expr> <minute|hour|day>
  <block>
```

### AI statements
```
ask ai "<prompt>"

ask ai
  provider "<name>"
  model "<name>"
  system "<prompt>"
  user "<prompt>"
  retries <n>

extract from <expr>:
  <field>
  <field>

extract from <expr> using ai:
  provider "<name>"
  model "<name>"
  system "<prompt>"
  fields:
    <field>
    <field>

classify <expr> as:
  <label>
  <label>

summarize <expr> into <name>

rewrite <expr> in <tone>

decide <name> from <expr>:
  <option>
  <option>
```

### MCP and tools
```
use mcp <target>
list tools
```

### Error handling
```
try
  <block>
catch <name>
  <block>

throw "<message>"

<expr> recover <fallback_expr>
```

### Context control
```
calculate tokens <expr>
compress <expr> into <name>
  target <fraction>          # or: auto target
  w_j <number>
  gain <number>
  mode structural|semantic|agent-safe
  preserve:
    <phrase>
  keep:
    first sentence
    last sentence
    headings
  require:
    <phrase>
```

### Agent
```
build agent <name>
  on webhook "<path>"
  use mcp <target>
  memory session
  provider "<name>"
  model "<name>"
  system "<prompt>"
  user <expr>
  reply <expr>
```

### Pipeline
```
<expr>
pipe <stage>
pipe <stage>
```

Supported pipeline stages: `summarize`, `rewrite`, `save`, `say`, `log`, function calls, tool calls.

---

## Expression forms

| Form | Example |
|---|---|
| String | `"hello {name}"` |
| Number | `42`, `3.14` |
| Boolean | `true`, `false` |
| Null | `null` |
| Identifier | `lead` |
| Property access | `body.email` |
| List | `["a", "b", "c"]` |
| Object | `{name: "Ana", total: 14}` |
| Read file | `read file "notes.txt"` |
| Load JSON | `load json "data.json"` |
| Load CSV | `load csv "leads.csv"` |
| HTTP GET | `get "http://..."` |
| HTTP POST | `post "http://..." with {<body>}` |
| HTTP PUT | `put "http://..." with {<body>}` |
| HTTP DELETE | `delete "http://..."` |
| Environment variable | `env "API_KEY"` |
| Current time | `now` |
| Random number | `random` |
| Length | `length <expr>` |
| Token count | `calculate tokens <expr>` |
| Function call | `call function greet with {name: "Ana"}` |
| Tool call | `call tool "filesystem.read_file" with {path: "f.txt"}` |
| Tool list | `list tools` |
| Recover fallback | `<expr> recover <fallback>` |

---

## Operators

```
+    addition / string concatenation
==   equal
!=   not equal
>    greater than
>=   greater than or equal
<    less than
<=   less than or equal
```

---

## HTTP response shape

HTTP expressions return objects with these fields:

```
response.status        # integer status code
response.status_text   # status string
response.headers       # headers object
response.body          # parsed body (JSON object or raw string)
response.text          # raw response text
```

---

## The `result` variable

Many AI and structured operations write their output into `result` when no explicit target variable is set. Assign it promptly after use to avoid ambiguity in longer scripts.

```glow
classify message as:
  urgent
  normal
  spam

set priority to result   # capture immediately
```
