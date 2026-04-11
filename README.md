# cuda-context-window

Context window management — compression, summarization, and token budgeting for agent conversations

Part of the Cocapn memory layer — how agents remember, forget, and recall.

## What It Does

### Key Types

- `ContextMessage` — core data structure
- `ContextWindow` — core data structure
- `CompactionAdvice` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-context-window.git
cd cuda-context-window

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_context_window::*;

// See src/lib.rs for full API
// 5 unit tests included
```

### Available Implementations

- `ContextWindow` — see source for methods

## Testing

```bash
cargo test
```

5 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: memory
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-memory-fabric](https://github.com/Lucineer/cuda-memory-fabric)
- [cuda-temporal](https://github.com/Lucineer/cuda-temporal)
- [cuda-adaptation](https://github.com/Lucineer/cuda-adaptation)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
