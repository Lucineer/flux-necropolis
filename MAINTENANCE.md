# Maintenance

## Running Tests

```bash
cargo test
```

## Adding New VesselState Variants

1. Add variant to `VesselState` in `src/tombstone.rs`
2. Add a `count_*` method to `Graveyard` in `src/graveyard.rs`
3. Update `Necropolis::statistics()` if the variant should be tracked
4. Add tests in `src/lib.rs`

## Capacity Limits

| Resource | Limit | Constant |
|---|---|---|
| Graveyard | 64 tombstones | `GRAVEYARD_MAX` |
| Afterlife | 128 fragments | `AFTERLIFE_MAX` |

These are compile-time constants. To change, update the constant in the respective module.

## Module Dependencies

```
tombstone ← graveyard ← necropolis
tombstone ← afterlife ← necropolis
         ← memorial ← necropolis
```

No external dependencies. Zero-cost abstractions only.
