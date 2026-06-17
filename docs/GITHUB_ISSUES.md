

---

## Issue 1 — Add `cargo-deny` to the main CI pipeline

**Labels:** `ci`, `security`, `good first issue`

**As a** Maintainer,  
**I want** the main CI workflow to run `cargo-deny` on every PR  
**So that** disallowed licenses and known vulnerable dependencies are blocked before merge.

### Description

`deny.toml` exists at the repo root and defines license and advisory policies, but `.github/workflows/ci.yml` does **not** run `cargo-deny`. A stale backup workflow referenced it, but that file was removed. Security scanning should be part of the canonical CI path.

### ✅ Requirements

- Add a `cargo-deny` job to `.github/workflows/ci.yml` (or a dedicated `security.yml` workflow triggered on the same events).
- Use [EmbarkStudios/cargo-deny-action](https://github.com/EmbarkStudios/cargo-deny-action) with `command: check` and `--config deny.toml`.
- Run from the `neurowealth-vault` workspace (or repo root if `deny.toml` paths are adjusted).
- Document the exception process in `CONTRIBUTING.md` (link to comments already in `deny.toml`).

### 🎯 Acceptance Criteria

- [ ] CI fails when a dependency violates `deny.toml` rules.
- [ ] CI passes on current `main` with no policy changes required.
- [ ] `CONTRIBUTING.md` explains how to request a time-limited advisory exception.

---

## Issue 2 — Enforce optimised WASM size limit in CI

**Labels:** `ci`, `performance`, `good first issue`

**As a** Maintainer,  
**I want** CI to fail when the optimised contract WASM exceeds a configured byte limit  
**So that** we catch binary bloat before it blocks Stellar deployment.

### Description

`docs/WASM_SIZE.md` documents a **1.5 MB** gate, but the current `.github/workflows/ci.yml` `build-wasm` step only builds and runs `wasm-opt` — it does **not** assert a size limit. Stellar contracts have practical size constraints; an automated gate prevents regressions.

### ✅ Requirements

- After `wasm-opt`, measure the output `.wasm` file size.
- Fail the job if size exceeds `WASM_SIZE_LIMIT_BYTES` (default: `1500000`).
- Print size and limit in the GitHub Actions step summary.
- Upload the optimised WASM as a workflow artifact (optional but recommended).
- Fix `docs/WASM_SIZE.md`: it currently references **NEAR** limits; update for **Stellar/Soroban** deployment constraints.

### 🎯 Acceptance Criteria

- [ ] CI fails when optimised WASM exceeds the limit.
- [ ] CI passes on current `main`.
- [ ] `docs/WASM_SIZE.md` accurately describes Stellar/Soroban (not NEAR).
- [ ] Limit is configurable via workflow `env` without code changes.

---

## Issue 3 — Add access-control test: non-owner cannot call `set_deposit_limits`

**Labels:** `testing`, `security`, `good first issue`

**As a** Developer,  
**I want** a regression test proving a non-owner cannot update deposit limits  
**So that** owner-only configuration cannot be bypassed silently.

### Description

`set_deposit_limits` is owner-only via `require_is_owner`. `test_access_control.rs` covers owner happy paths and non-owner rejection for `pause`, `set_blend_pool`, and `upgrade`, but **not** for `set_deposit_limits` or `set_tvl_cap`.

### ✅ Requirements

- Add `test_non_owner_cannot_set_deposit_limits` in `neurowealth-vault/contracts/vault/src/tests/test_access_control.rs`.
- Follow the existing pattern: `setup_vault_with_token`, call as `Address::generate`, expect panic.
- Optionally add `test_non_owner_cannot_set_tvl_cap` and `test_non_owner_cannot_set_limits` for parity.

### 🎯 Acceptance Criteria

- [ ] `#[should_panic]` test fails if owner check is removed from `set_deposit_limits`.
- [ ] `cargo test -p neurowealth-vault` passes.
- [ ] Test uses explicit owner vs non-owner addresses (not `mock_all_auths` only).

---

## Issue 4 — Split `LimitsUpdatedEvent` into semantic admin events

**Labels:** `contract`, `events`, `breaking-change`

**As an** Indexer / AI Agent developer,  
**I want** distinct events for TVL cap, user deposit cap, and per-deposit min/max changes  
**So that** off-chain systems can subscribe to the correct event type without misinterpreting fields.

### Description

Today `LimitsUpdatedEvent { old_min, new_min, old_max, new_max }` is reused for:

| Function | What `old_min` / `new_min` actually mean |
|----------|------------------------------------------|
| `set_tvl_cap` | User deposit cap (unchanged) / unchanged |
| `set_user_deposit_cap` | User deposit cap old/new |
| `set_limits` | User deposit cap old/new |
| `set_deposit_limits` | Min deposit old/new |

The `old_max` / `new_max` fields similarly map to TVL cap or max deposit depending on the caller. This is confusing for indexers documented in `EVENTS.md`.

### ✅ Requirements

- Introduce focused events, e.g.:
  - `TvlCapUpdatedEvent { old_cap, new_cap }`
  - `UserDepositCapUpdatedEvent { old_cap, new_cap }`
  - `DepositLimitsUpdatedEvent { old_min, new_min, old_max, new_max }` (for `set_deposit_limits` only)
- Update `EVENTS.md`, `contract-spec.json` generation, and event schema tests.
- Deprecate or remove overloaded `LimitsUpdatedEvent` usage (coordinate with agent/indexer consumers).

### 🎯 Acceptance Criteria

- [ ] Each admin limit function emits an event whose fields match its semantics.
- [ ] `test_event_schema.rs` and `test_events.rs` updated.
- [ ] `EVENTS.md` documents topic symbols and payload fields per event.
- [ ] No breaking ambiguity in event field meaning.

---

## Issue 5 — On-chain per-user investment strategy preference

**Labels:** `contract`, `enhancement`, `phase-2`

**As a** User,  
**I want** my chosen strategy (Conservative / Balanced / Growth) stored on-chain  
**So that** the AI agent can honor strategy preference without relying solely on off-chain database state.

### Description

The README and product vision describe three strategies. The contract supports `rebalance(protocol, expected_apy, min_out)` at the vault level but has **no per-user strategy** field. Strategy is currently an agent/DB concern, which creates a trust and recovery gap if the agent DB is lost or compromised.

### ✅ Requirements

- Add `DataKey::UserStrategy(Address)` (or enum in persistent storage).
- Add `set_user_strategy(env, user, strategy)` — user must `require_auth()`.
- Add `get_user_strategy(env, user) -> Symbol`.
- Emit `UserStrategyUpdatedEvent { user, old_strategy, new_strategy }`.
- Default strategy on first deposit (e.g. `balanced`) — document in `ARCHITECTURE.md`.
- Unit tests for set/get, auth, and event emission.

### 🎯 Acceptance Criteria

- [ ] User can set strategy; only that user can change their own strategy.
- [ ] Agent can read strategy via contract view call.
- [ ] Event emitted on change; documented in `EVENTS.md`.
- [ ] Tests cover all three strategy symbols and unauthorized updates.

---

## Issue 6 — DEX liquidity pool integration (Phase 2)

**Labels:** `contract`, `defi`, `phase-2`

**As a** Developer,  
**I want** the vault to support deploying USDC to Stellar DEX liquidity pools in addition to Blend  
**So that** the Balanced and Growth strategies described in the README can be executed on-chain.

### Description

Blend integration exists (`supply_to_blend`, `withdraw_from_blend`, `BlendPool` storage). README lists DEX liquidity for Balanced/Growth strategies, but `rebalance` only moves funds to/from Blend today. `get_protocol_balance` returns 0 for non-Blend protocols.

### ✅ Requirements

- Research and document DEX pool interface in `BLEND_INTEGRATION_RESEARCH.md` or a new `DEX_INTEGRATION.md`.
- Add `DataKey::DexPool` (or protocol registry) and owner-configurable pool address(es).
- Implement `supply_to_dex` / `withdraw_from_dex` internal helpers mirroring Blend patterns.
- Extend `rebalance` to support a `dex` (or named) protocol symbol with `min_out` slippage protection.
- Integration tests with mock DEX contract (same pattern as `test_blend_integration.rs`).
- Optional: `blend-devnet`-style feature flag for testnet DEX smoke tests.

### 🎯 Acceptance Criteria

- [ ] Agent can rebalance vault USDC into and out of a configured DEX pool.
- [ ] `CurrentProtocol` and `ProtocolChangedEvent` reflect DEX deployments.
- [ ] Slippage floor (`min_out`) enforced on DEX legs.
- [ ] Tests pass without network; devnet tests documented if behind feature flag.

---

## Issue 7 — Generate TypeScript client from `contract-spec.json`

**Labels:** `tooling`, `agent`, `frontend`, `good first issue`

**As a** Frontend / Agent developer,  
**I want** a typed TypeScript client generated from `contract-spec.json`  
**So that** the planned agent and web app can invoke vault functions without hand-maintaining bindings.

### Description

CI generates and validates `contract-spec.json` via `scripts/generate-spec.py`. There is no generated client package for `@stellar/stellar-sdk` consumers. README describes `agent/` and `frontend/` as planned — typed bindings reduce integration errors.

### ✅ Requirements

- Add a script (Node or Python) that reads `contract-spec.json` and emits TypeScript types + invoke helpers.
- Output to `packages/vault-client/` (or `agent/src/generated/vault.ts`) with README.
- Wire generation into `.github/workflows/contract-spec.yml` (fail if generated output is stale on PR).
- Document usage in `scripts/README-SPEC.md`.

### 🎯 Acceptance Criteria

- [ ] Running the generator produces TypeScript that matches current spec.
- [ ] CI fails if spec changes but generated client is not updated.
- [ ] Example snippet shows invoking `deposit` and `get_balance` from TypeScript.

---

## Issue 8 — Align README project structure with the actual repository

**Labels:** `documentation`, `good first issue`

**As a** New contributor,  
**I want** the README project tree to match the real repo layout  
**So that** I can find contracts, scripts, and docs without confusion.

### Description

README shows:

```
neurowealth/
├── contracts/
├── agent/          # [Planned]
├── frontend/       # [Planned]
```

The actual repo is `NeuroWealth-Smartcontract/` with `neurowealth-vault/`, `scripts/`, `docs/`, and no top-level `agent/` or `frontend/` yet. Getting Started paths reference `cd contracts` but the workspace is `neurowealth-vault/`.

### ✅ Requirements

- Update README **Project Structure** to reflect current directories.
- Fix **Build and Deploy** commands to use `neurowealth-vault` paths and Stellar CLI (`stellar` not legacy `soroban` where applicable).
- Mark planned directories clearly as **not yet in repo** with links to tracking issues.
- Cross-link `ARCHITECTURE.md`, `CONTRIBUTING.md`, and `scripts/README-E2E.md`.

### 🎯 Acceptance Criteria

- [ ] A new developer can build and test by following README commands only.
- [ ] No references to non-existent paths (`contracts/` at repo root).
- [ ] Planned vs implemented components are clearly labeled.

---

## Issue 9 — Automate mainnet preflight checks in `scripts/verify-deployment.sh`

**Labels:** `tooling`, `mainnet`, `devops`

**As a** Maintainer,  
**I want** a script that validates post-deploy vault configuration against `docs/MAINNET_CHECKLIST.md`  
**So that** mainnet launches are repeatable and less error-prone.

### Description

`docs/MAINNET_CHECKLIST.md` is comprehensive but manual. `scripts/verify-deployment.sh` exists but should be extended to assert checklist items automatically where possible.

### ✅ Requirements

- Accept `VAULT_CONTRACT_ID` and `NETWORK` env vars.
- Assert via contract invocations:
  - `get_owner() != get_agent()`
  - Vault is initialized, not paused
  - `MinDeposit`, `MaxDeposit`, `TvLCap`, `UserDepositCap` match expected env/config
  - Blend pool address set if yield deployment is enabled
- Print pass/fail report; exit non-zero on any failure.
- Document in `docs/MAINNET_CHECKLIST.md` with copy-paste commands.

### 🎯 Acceptance Criteria

- [ ] Script runs against testnet with documented example contract ID.
- [ ] Fails loudly when owner == agent or caps are unset.
- [ ] Checklist doc references the script for each automatable step.

---

## Issue 10 — Run fuzz tests on PRs touching `deposit` / `withdraw` math

**Labels:** `ci`, `testing`, `security`

**As a** Maintainer,  
**I want** the `deposit_withdraw_sequence` fuzz target to run on relevant PRs  
**So that** share accounting regressions are caught before merge.

### Description

`neurowealth-vault/fuzz/` contains `deposit_withdraw_sequence`. CI only runs fuzz on a **weekly schedule**, not on PRs that modify deposit/withdraw/share math — when failures matter most.

### ✅ Requirements

- Add a CI job (or extend `ci-matrix`) that runs `cargo fuzz run deposit_withdraw_sequence` with bounded `-runs` and `-max_total_time` on PRs when paths under `neurowealth-vault/contracts/vault/src/` change.
- Use nightly Rust toolchain (required for `cargo-fuzz`).
- Keep the weekly schedule for longer runs; PR job uses shorter bounds for speed.

### 🎯 Acceptance Criteria

- [ ] PR modifying vault `lib.rs` or share math tests triggers fuzz job.
- [ ] Job completes within CI timeout (e.g. 10 minutes).
- [ ] Document local fuzz workflow in `CONTRIBUTING.md`.

---

## Files removed in this cleanup (reference)

The following stale or duplicate files were removed from the repo:

| File | Reason |
|------|--------|
| `pr.md`, `PR_BODY_149-152.md` | Merged PR bodies; not source of truth |
| `AUDIT_COMPLETION_SUMMARY.md` | One-time audit snapshot; use `SECURITY.md` + issues |
| `CONTRACT_SPEC_GENERATION_SUMMARY.md` | Duplicates `scripts/README-SPEC.md` |
| `neurowealth-vault/fix.py` | One-off local hack script |
| `neurowealth-vault/contracts/vault/src/test.rs` | Orphaned; tests live in `src/tests/` |
| `scripts/generate-contract-spec.rs` | Unused duplicate of `generate-spec.py` |
| `.github/workflows/backup/*` | Stale backup workflows |
| `.github/workflows/.github/workflows/ci.yml` | Accidental nested path; wrong `contracts/vault` paths |
| `.kiro/specs/**` | AI tooling metadata, not for contributors |
| `docs/GITHUB_ISSUE_CI_CD.md` | Empty placeholder |
| `.github/ISSUE_TEMPLATE/ci-cd-soroban.md` | CI already implemented in `ci.yml` |
