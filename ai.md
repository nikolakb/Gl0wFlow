# GlowFlow AI

GlowFlow exposes GlowScript `ask ai` as a first-class language feature.

This is central to GlowFlow's positioning around GlowScript as the scripting language for AI automation.

GlowScript also includes structured AI-native statements:

- `extract from text:`
- `classify message as:`
- `summarize report into short_summary`
- `rewrite reply in friendly tone`
- `decide next_step from context:`

Block configuration is also supported, for example:

```glow
ask ai
  provider "openai"
  model "gpt-5"
  system "You extract customer details"
  user "Name: Ana email ana@example.com"
```

And structured forms can carry AI config too:

```glow
extract from message using ai:
  provider "openai"
  model "gpt-5"
  system "Extract customer details"
  fields:
    name
    email
```

Current MVP behavior:

- GlowScript supports explicit AI providers in config: `command`, `mock`, `openai`, and `ollama`
- if `provider "command"` is selected, or `GLOW_AI_COMMAND` is set without an explicit provider, GlowScript will execute that command and pass the prompt through stdin and `GLOW_AI_PROMPT`
- structured tasks also expose:
  - `GLOW_AI_TASK`
  - `GLOW_AI_PROMPT`
  - `GLOW_AI_OPTIONS`
  - `GLOW_AI_SCHEMA`
  - `GLOW_AI_RESPONSE_FORMAT`
  - `GLOW_AI_MODEL`
  - `GLOW_AI_SYSTEM`
- `provider "mock"` reads `GLOW_AI_MOCK_RESPONSE`
- `provider "openai"` uses `GLOW_OPENAI_API_KEY` and optional `GLOW_OPENAI_BASE_URL`
- `provider "ollama"` uses optional `GLOW_OLLAMA_BASE_URL`
- `retries N` retries provider execution up to `N` extra times
- otherwise, GlowScript returns a deterministic stub response for development

Current provider proof:

- GlowFlow now has real Ollama-backed proof runs for `extract`, `classify`, `decide`, and `summarize`
- the current verified local server model was `qwen3-coder:480b-cloud`
- this was served through a local Ollama instance, but the available model itself was cloud-backed rather than a fully local on-disk model
- GlowFlow now also normalizes fenced JSON responses from providers before validating structured output contracts

Typed response contracts:

- `extract` expects an object response with all requested fields
- `classify` expects exactly one allowed label
- `decide` expects exactly one allowed option

This means adapters can validate outputs against an explicit contract instead of relying only on prompt wording.
