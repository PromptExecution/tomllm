# tomllm

TOML + LLM comment conventions: tribal knowledge in `#comments`, stripped for data pipelines.

## Overview

`.tomllm` files are **valid TOML** with enriched `#` comment semantics:

- Comments associate with the next key-value pair or section header
- Special prefixes encode tribal knowledge: `# 🤓`, `# @tribal:`, `# @example:`, `# @requires:`
- Tail-map block (last ≤10 lines): fast executive agent scanning without full context load
- Comments are FOR agents reading the **source file** as documentation — strip them for downstream pipelines

## Cognitive Tiers

Each `.tomllm` file MAY declare its required cognitive tier in the tail-map:

| Tier | Models | Tasks |
|------|--------|-------|
| `sm0l` | qwen2.5-3B, haiku | classify, route, grep, format |
| `ch0nky` | qwen3-coder (local) | implement, refactor, debug |
| `frontier` | claude-opus/sonnet | architecture, security, novel design |

## Example

```toml
# @tribal: always use uv pip, never pip install directly
# @example: uv pip install requests
package_manager = "uv"

# b00t:map v1
# summary: Python toolchain config
# tags: python, uv
# tier: sm0l
# complexity: 2
```

## Usage

### Rust

```rust
use tomllm::TomllmDoc;

let doc = TomllmDoc::parse(input)?;
let clean_json = doc.strip_for_pipeline(); // stripped TOML as JSON
let tier = doc.cognitive_tier();           // sm0l / ch0nky / frontier
let map = doc.map_block;                   // Option<MapBlock>
```

### Python (maturin)

```python
from tomllm import TomllmDoc, MapBlock

doc = TomllmDoc.parse(text)
print(doc.cognitive_tier())   # "sm0l"
print(doc.strip_for_pipeline())  # JSON string

block = MapBlock.from_text(text)
if block:
    print(block.summary, block.tier, block.tags)
```

### TypeScript / WASM

```typescript
import init, { TomllmDoc } from '@promptexecution/tomllm';

await init();
const doc = TomllmDoc.parse(text);
```

## Install

```bash
# Rust
cargo add tomllm

# Python
pip install tomllm

# npm
npm install @promptexecution/tomllm
```

## Tail-map format

```toml
# b00t:map v1
# summary: one-line human+LLM description
# tags: comma, separated, keywords
# tier: sm0l|ch0nky|frontier
# cmds: b00t hive activate inference-qwen3, b00t hive status
# complexity: 1-10
```

## License

MIT

<!-- b00t:map v1
summary: tomllm — TOML + LLM comment conventions crate
tags: toml, llm, agent, config, annotations, wasm, python
tier: sm0l
cmds: cargo test, maturin build --features python, wasm-pack build
complexity: 3
-->
