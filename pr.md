## Summary

Closes #225 — adds regression tests proving non-owners cannot call `set_deposit_limits`, `set_tvl_cap`, and `set_limits`.

- **`test_non_owner_cannot_set_deposit_limits`** — required test from the issue; verifies `set_deposit_limits` rejects calls without owner auth.
- **`test_non_owner_cannot_set_tvl_cap`** — parity test; verifies `set_tvl_cap` rejects calls without owner auth.
- **`test_non_owner_cannot_set_limits`** — parity test; verifies `set_limits` rejects calls without owner auth.

## Why a different pattern from the existing non-owner tests

The existing negative tests (`test_non_owner_cannot_pause`, `test_non_owner_cannot_upgrade`, etc.) work by passing a freshly-generated `Address::generate` to a function that takes an explicit owner parameter and does an identity comparison. Those functions can be called "as a non-owner" because there's a param to swap.

`set_deposit_limits`, `set_tvl_cap`, and `set_limits` have no such parameter — ownership is enforced via `require_is_owner`, which fetches the stored owner address and calls `owner.require_auth()`. The only way to test rejection is to ensure the stored owner's auth is absent at call time.

The tests follow the stricter pattern already used in `test_auth.rs`:
1. Call `env.mock_all_auths()` during setup (initialization legitimately requires the deployer's signature — mocking it is incidental setup noise).
2. Call `env.mock_auths(&[])` immediately before the function under test, revoking all authorization.
3. Use the `try_*` client variant and `assert!(result.is_err())`.

If the `require_is_owner` guard is ever removed from any of these functions, valid inputs would complete successfully, `result.is_err()` would be `false`, and the test would fail — exactly what the issue requires.

## Test plan

- [ ] `cargo test -p neurowealth-vault test_non_owner_cannot_set_deposit_limits` passes
- [ ] `cargo test -p neurowealth-vault test_non_owner_cannot_set_tvl_cap` passes
- [ ] `cargo test -p neurowealth-vault test_non_owner_cannot_set_limits` passes
- [ ] No existing tests in `test_access_control.rs` are broken
- [ ] Removing `require_is_owner` from any of the three functions causes the corresponding test to fail
