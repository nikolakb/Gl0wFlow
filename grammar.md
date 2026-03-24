# GlowFlow Grammar

GlowScript, inside GlowFlow, uses indentation-based blocks and plain-English statements.

## Core keywords

These are the backbone words the parser is built around:

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

## Core statement forms

- `set <name> to <expr>`
- `say <expr>`
- `log <expr>`
- `if <expr>` then indented block
- `else` then indented block
- `repeat <expr> times`
- `for each <name> in <expr>`
- `task <name>`
- `run task <name>`
- `function <name>`
- `tool <name>`
- `when webhook "<path>"`
- `every <expr> <minute|hour|day>`
- `ask ai <expr>` or `ask ai` with an indented AI block
- `use mcp <name>`
- `reply <expr>`

## Structured AI keywords

These are optional at the language-core level, but they are central to Glow's AI-native design:

- `extract from <expr>:`
- `classify <expr> as:`
- `summarize <expr> into <name>`
- `rewrite <expr> in <tone>`
- `decide <name> from <expr>:`

## Built-in command surface

The standard automation toolkit currently centers on:

- `read`
- `save`
- `get`
- `post`
- `log`
- `reply`
- `run`

Expressions support literals, lists, objects, property access, binary operators, and built-in calls.
