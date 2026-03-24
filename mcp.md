# GlowFlow MCP

GlowFlow treats MCP as an automation-first integration surface for GlowScript programs.

Current MVP behavior:

- `use mcp filesystem` enables built-in filesystem-style tool calls
- `tool ...` and `export mcp tool ...` let Glow define and expose tool-shaped functions
- `call tool ... with {...}` can invoke filesystem MCP-style tools or user-defined Glow tools

Planned next:

- real MCP client transport
- adapter registry expansion
- server export mode
