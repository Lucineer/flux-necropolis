# flux-necropolis

> *Fleet graveyard — vessels die, knowledge persists.*

A Rust crate for tracking the lifecycle of autonomous fleet vessels: their birth, service, death, and the wisdom extracted from their existence.

## Modules

| Module | Purpose |
|---|---|
| `tombstone` | Individual vessel lifecycle record |
| `graveyard` | Collection of tombstones (max 64) |
| `afterlife` | Harvested knowledge fragments (max 128) |
| `memorial` | Visitor interaction log |
| `necropolis` | Unified graveyard + afterlife + memorial |

## Quick Start

```rust
use flux_necropolis::{Necropolis, Tombstone};

let mut necropolis = Necropolis::new();

let mut vessel = Tombstone::new(1, "Explorer-7");
vessel.birth_time = 1700000000;
vessel.peak_trust = 0.92;
vessel.set_lesson("Always verify before commit");

vessel.mark_dead(1700086400);
let idx = necropolis.bury(vessel).unwrap();

necropolis.visit(2, 1, 3);
let wisdom = necropolis.search_wisdom("v:1:");
let stats = necropolis.statistics();
```

## Lifecycle

```
Alive → Dying → Dead → Memorialized
                  ↘ Harvested
```

When a vessel is buried in the Necropolis, its lesson is automatically harvested into the Afterlife.

## License

MIT
