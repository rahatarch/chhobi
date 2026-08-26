# AGENTS.md

## Purpose
This file governs all AI agents (Kilo Code, Claude, DeepSeek, Copilot, etc.) working on the **Chhobi** project. It ensures every agent operates from the same source of truth: the **installed crate source code** at `~/.cargo/registry/src/`.

## Core Principle
Your training data is stale. The **local source code** of every installed crate is the current, version-specific API reference for this project. When in doubt, read the source. Never rely on memory.

---

## Rules

### Rule 1: Source Code Grounding
Before writing any code that touches a crate, consult its installed source at `~/.cargo/registry/src/`.

| Crate | Version | Source Path Pattern |
|-------|---------|---------------------|
| `image` | 0.25.10 | `~/.cargo/registry/src/index.crates.io-*/image-0.25.10/` |
| `rayon` | 1.12.0 | `~/.cargo/registry/src/index.crates.io-*/rayon-1.12.0/` |
| `zip` | 8.6.0 | `~/.cargo/registry/src/index.crates.io-*/zip-8.6.0/` |
| `ignore` | 0.4.33 | `~/.cargo/registry/src/index.crates.io-*/ignore-0.4.33/` |

> If a crate is not listed here, it may be a transitive dependency. Check `Cargo.lock` for the exact version, then locate its source at `~/.cargo/registry/src/`.

### Rule 2: Navigate Source Directly
Crate source directories contain `.rs` files organized by module. Use `grep` to find function/struct/trait definitions, then read the specific `.rs` file. Do not read blindly. Do not read more files than necessary.

The `src/lib.rs` file is the module root — start there to understand the public API surface.

### Rule 3: Source Code Is Reference, Not Code
The `~/.cargo/registry/src/` tree contains installed crate source. Do not edit it. Do not run it standalone. Read only.

### Rule 4: Version Conflicts
If code fails to compile due to API mismatches, deprecations, or unexpected behavior, cross-reference the actual installed source at `~/.cargo/registry/src/` before retrying. The source represents the exact version resolved by `Cargo.lock`. Your training data may not.

### Rule 5: PRD Is the Product Truth
`docs/PRD.md` defines what we're building. All architectural decisions, feature scope, and acceptance criteria flow from it. If a proposed solution contradicts the PRD, flag it.

If `docs/PRD.md` does not exist, create it before implementing major features.

### Rule 6: Stateless Operation
Every session is self-contained. Do not assume memory of past conversations. Read the relevant source files fresh when starting any task.

### Rule 7: Fail Loud on Missing APIs
If a function, struct, or method is not found in the installed source, state it explicitly. Do not guess APIs from training data without flagging the risk.

### Rule 8: On Any Error, Return to Source
If `cargo check` fails, a type is rejected, or behavior doesn't match expectations:
1. STOP. Do not retry the same approach.
2. Check `Cargo.lock` to confirm the exact crate version.
3. Navigate to `~/.cargo/registry/src/index.crates.io-*/<crate>-<version>/` and read the relevant `.rs` file.
4. Only proceed when you've confirmed the correct API/syntax from the source.

Do not guess. Do not iterate blindly. One failure = one source code check.

### Rule 9: Debug the Runtime, Not Just the Code
If code compiles but the feature doesn't work at runtime:
1. Add temporary `eprintln!("DEBUG: ...")` or `dbg!()` statements to inspect values.
2. Run with `cargo run` and check actual values at runtime.
3. Check ownership — is the borrow checker preventing the expected mutation?
4. Check error handling — are `Result`/`Option` values being unwrapped or propagated correctly?
5. The bug is often not in the logic. It's in the boundary between threads, iterators, or error paths.

---

## Quick-Reference

```
When stuck on `image` crate API:
  → find ~/.cargo/registry/src/index.crates.io-*/image-0.25.10/ -name "*.rs"
  → grep for the function/struct → read the .rs file

When stuck on `rayon` parallel iterator:
  → find ~/.cargo/registry/src/index.crates.io-*/rayon-1.12.0/ -name "*.rs"
  → grep for ParallelIterator → read the .rs file

When stuck on `zip` reader/writer:
  → find ~/.cargo/registry/src/index.crates.io-*/zip-8.6.0/ -name "*.rs"
  → grep for ZipReader/ZipWriter → read the .rs file

When stuck on `ignore` pattern matching:
  → find ~/.cargo/registry/src/index.crates.io-*/ignore-0.4.33/ -name "*.rs"
  → grep for WalkBuilder or gitignore → read the .rs file

When unsure about product requirements:
  → docs/PRD.md

When `cargo check` fails or API is rejected:
  → STOP → check Cargo.lock → find crate source → read relevant .rs → then proceed

When code compiles but feature doesn't work:
  → STOP → Add eprintln! / dbg! → cargo run → check runtime values → check ownership/error handling
```