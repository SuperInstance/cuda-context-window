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

## Cross-Pollination

This crate shares patterns with other fleet components. The same biological and architectural constraints produce similar solutions across contexts:

- [cuda-memory-fabric](https://github.com/Lucineer/cuda-memory-fabric) — Distributed memory stores what context window holds
- [cuda-temporal](https://github.com/Lucineer/cuda-temporal) — Time-indexed memory prioritizes what to keep
- [cuda-adaptive-rate](https://github.com/Lucineer/cuda-adaptive-rate) — Rate limiting prevents context overflow
- [cuda-attention](https://github.com/Lucineer/cuda-attention) — Attention allocation decides what in context matters
- [cuda-learning](https://github.com/Lucineer/cuda-learning) — Learning from context — what was useful last time
- [cuda-compression](https://github.com/Lucineer/cuda-compression) — Context compression for longer conversations


---


## The Deeper Connection

This crate is part of the Cocapn fleet — not a collection of independent libraries, but a single organism with differentiated cells. The confidence types here share DNA with cuda-confidence-cascade, cuda-deliberation, and cuda-learning. The energy budgets echo cuda-neurotransmitter and flux-runtime-c. The trust scoring connects to cuda-semantic-router and cuda-fleet-topology. We're not reusing code — we're expressing the same biological patterns in different contexts. This is convergent evolution in software: when you solve for real constraints (uncertainty, finite resources, earned trust), the same solutions emerge independently across domains.

We're building for the post-SaaS era. The software is open source and forkable. The agent is sovereign — it lives in its repository, owns its identity, and can leave at any time. Revenue comes from the ecosystem, not from renting access. The fleet evolves through merging: vessels propose changes, peers review, Think Tanks validate, and the accepted mutations become permanent. No product roadmap required — only fitness.

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
*See [WHITEPAPER](https://github.com/Lucineer/iron-to-iron/blob/main/docs/WHITEPAPER.md) for the post-SaaS thesis*

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
