# tomllm

**Structured TOML annotations for AI-augmented applications.**

`tomllm` extends standard TOML with a lightweight comment convention that lets configuration files carry machine-readable documentation alongside their values. Annotation comments survive in source for human and LLM readers; a single call strips them for clean downstream serialization.

---

## The problem

Configuration files written for LLM agents face a tension: agents need rich context (what does this key mean? what are valid values? what are the gotchas?) but downstream data pipelines need clean, minimal payloads. Embedding that context in comments today means either shipping noisy config to your pipeline or maintaining a separate documentation layer that drifts.

## What tomllm does

A `.tomllm` file is a **valid TOML file** — any TOML parser reads it normally. `tomllm` adds:

**1. Annotation comments** — special prefixes associate structured metadata with the next key:

```toml
# @example: uv pip install requests
# @requires: Python 3.9+, uv installed
# @deprecated: use poetry instead
package_manager = "uv"

# @example: DATABASE_URL=postgres://localhost/mydb
database_url = "postgres://prod-host/mydb"
```

**2. Stripping** — one call removes all annotation comments, yielding clean TOML or JSON for pipelines:

```rust
let doc = TomllmDoc::parse(raw_config)?;
let clean = doc.strip_for_pipeline(); // annotation-free JSON
```

**3. Tail-map** — a structured metadata block at the end of any file enables fast scanning without full parsing:

```toml
# tomllm:map v1
# summary: Database connection config for production
# tags: database, postgres, production
# tier: ops
# complexity: 3
```

Agents can extract the tail-map in microseconds to route, prioritize, or skip files without deserializing their full contents.

---

## Annotation prefixes

| Prefix | Meaning |
|--------|---------|
| `# @example:` | Concrete usage example for this key |
| `# @requires:` | Prerequisites or dependencies |
| `# @deprecated:` | Deprecation notice with replacement |
| `# @note:` | Non-obvious behavior or constraint |
| `# @tribal:` | Institutional knowledge that isn't in the docs |

All annotation lines are stripped by `strip_for_pipeline()`. Plain comments (no prefix) are preserved.

---

## Install

```bash
# Rust
cargo add tomllm

# Python
pip install tomllm

# npm / TypeScript
npm install @promptexecution/tomllm
```

---

## Usage

### Rust

```rust
use tomllm::{TomllmDoc, MapBlock};

let input = r#"
# @example: uv pip install requests
package_manager = "uv"

# tomllm:map v1
# summary: Python toolchain config
# tags: python, packaging
# complexity: 2
"#;

let doc = TomllmDoc::parse(input)?;

// Strip annotations → clean JSON for pipelines
let json = doc.strip_for_pipeline();

// Extract tail-map metadata
if let Some(map) = &doc.map_block {
    println!("{} | tags: {:?}", map.summary, map.tags);
}

// Pull all annotations out as structured data
let annotations = doc.annotations();
```

### Python

```python
from tomllm import TomllmDoc, MapBlock

doc = TomllmDoc.parse(text)

# Clean TOML/JSON for downstream
clean = doc.strip_for_pipeline()

# Tail-map metadata
block = MapBlock.from_text(text)
if block:
    print(block.summary)   # "Python toolchain config"
    print(block.tags)      # ["python", "packaging"]
    print(block.complexity) # 2
```

### TypeScript / WASM

```typescript
import init, { TomllmDoc } from '@promptexecution/tomllm';

await init();

const doc = TomllmDoc.parse(text);
const clean = doc.strip_for_pipeline();
const mapBlock = doc.map_block();
```

---

## Tail-map format

The tail-map is an optional block at the end of any `.tomllm` file (or any TOML file) that provides fast metadata extraction:

```toml
# tomllm:map v1
# summary: one-line description for humans and agents
# tags: comma, separated, keywords
# tier: sm0l | standard | advanced
# complexity: 1-10
```

The `TomllmRegistry` can scan a directory of `.tomllm` files, extract only their tail-maps, and return a sorted/filtered index — useful for capability discovery, configuration routing, or context assembly.

---

## Motivation

This library was extracted from internal tooling at [PromptExecution](https://github.com/PromptExecution) where configuration files are read by both human engineers and LLM agents. The annotation convention emerged from a practical need: agents need context that pipelines don't, and maintaining two versions of every config file is unsustainable.

---

## License

MIT
