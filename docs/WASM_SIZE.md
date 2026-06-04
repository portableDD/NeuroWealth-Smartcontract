# WASM Size Management

## CI Limit

The CI pipeline fails if the optimised contract WASM exceeds **1.5 MB** (configurable via `WASM_SIZE_LIMIT_BYTES` in `.github/workflows/ci.yml`).

NEAR Protocol's on-chain code-storage hard limit is **4 MB**.  Our 1.5 MB gate provides ample headroom and catches unintentional bloat early.

## Why This Matters

| Issue | Consequence |
|-------|-------------|
| WASM > 4 MB | Deployment transaction rejected by the network |
| WASM > CI limit | PR blocked until size is reduced |
| Gradual growth | Limits room for future feature additions |

## How to Reduce WASM Size

1. **Audit new dependencies** — `cargo bloat --release --crates` shows which crates contribute most to binary size.
2. **Use `no-default-features`** — disable crate features you don't need.
3. **Prefer `near-sdk` primitives** — avoid pulling in heavy `std` types where a simpler alternative exists.
4. **Avoid `format!` / `String` in hot paths** — string formatting pulls in significant code.
5. **Run `wasm-opt -Oz`** locally to see the post-optimisation size before pushing:
   ```bash
   cargo build --target wasm32-unknown-unknown --release --no-default-features
   wasm-opt -Oz --strip-debug --vacuum \
     target/wasm32-unknown-unknown/release/neurowealth_vault.wasm \
     -o /tmp/vault_opt.wasm
   wc -c /tmp/vault_opt.wasm
   ```

## Adjusting the Limit

If a deliberate feature addition requires a larger binary, update `WASM_SIZE_LIMIT_BYTES` in `ci.yml` in the same PR and document the reason in the PR description.