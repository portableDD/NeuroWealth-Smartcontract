/// Extended event-schema tests for neurowealth-vault
///
/// Replaces bare `!events.is_empty()` assertions with full payload
/// decoding, following the patterns established in test_event_schema.rs.
///
/// Tests that were already covered in the dedicated files under tests/
/// have been removed here to avoid redundancy.
use near_sdk::test_utils::{accounts, get_logs, VMContextBuilder};
use near_sdk::testing_env;
use serde_json::Value;

use crate::Contract;

// ── helpers ──────────────────────────────────────────────────────────────────

fn setup_owner() -> Contract {
    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(0));
    testing_env!(ctx.build());
    Contract::new(accounts(0))
}

/// Decode the first NEP-297 log whose `event` field matches `event_name`.
/// Panics with a descriptive message when not found.
fn find_event(logs: &[String], event_name: &str) -> Value {
    for log in logs {
        if let Some(json_str) = log.strip_prefix("EVENT_JSON:") {
            if let Ok(val) = serde_json::from_str::<Value>(json_str) {
                if val["event"] == event_name {
                    return val;
                }
            }
        }
    }
    panic!(
        "Event '{}' not found in logs.\nActual logs:\n{}",
        event_name,
        logs.join("\n")
    );
}

// ── deposit events ────────────────────────────────────────────────────────────

#[test]
fn test_deposit_event_payload() {
    let mut contract = setup_owner();

    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(1))
        .attached_deposit(1_000_000_000_000_000_000_000_000); // 1 NEAR
    testing_env!(ctx.build());

    contract.deposit();

    let logs = get_logs();
    let event = find_event(&logs, "vault_deposit");

    // Verify required fields are present and correctly typed
    assert_eq!(
        event["standard"], "nep297",
        "standard field must be 'nep297'"
    );
    assert!(
        event["data"][0]["account_id"].is_string(),
        "data[0].account_id must be a string"
    );
    assert!(
        event["data"][0]["amount"].is_string(),
        "data[0].amount must be a U128 string"
    );

    let amount: u128 = event["data"][0]["amount"]
        .as_str()
        .unwrap()
        .parse()
        .expect("amount must be parseable as u128");
    assert_eq!(
        amount, 1_000_000_000_000_000_000_000_000,
        "deposited amount must match attached deposit"
    );
}

// ── withdraw events ───────────────────────────────────────────────────────────

#[test]
fn test_withdraw_event_payload() {
    let mut contract = setup_owner();

    // Fund the account first
    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(1))
        .attached_deposit(2_000_000_000_000_000_000_000_000);
    testing_env!(ctx.build());
    contract.deposit();

    // Now withdraw half
    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(1));
    testing_env!(ctx.build());
    contract.withdraw(1_000_000_000_000_000_000_000_000.into());

    let logs = get_logs();
    let event = find_event(&logs, "vault_withdraw");

    assert_eq!(event["standard"], "nep297");
    assert!(event["data"][0]["account_id"].is_string());

    let amount: u128 = event["data"][0]["amount"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(amount, 1_000_000_000_000_000_000_000_000);
}

// ── rebalance events ──────────────────────────────────────────────────────────

#[test]
fn test_rebalance_event_payload() {
    let mut contract = setup_owner();

    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(0)); // only owner can rebalance
    testing_env!(ctx.build());
    contract.rebalance();

    let logs = get_logs();
    let event = find_event(&logs, "vault_rebalance");

    assert_eq!(event["standard"], "nep297");
    // Rebalance event must carry old and new allocations
    assert!(
        event["data"][0]["old_allocations"].is_object()
            || event["data"][0]["old_allocations"].is_array(),
        "old_allocations must be present"
    );
    assert!(
        event["data"][0]["new_allocations"].is_object()
            || event["data"][0]["new_allocations"].is_array(),
        "new_allocations must be present"
    );
}

// ── yield events ──────────────────────────────────────────────────────────────

#[test]
fn test_yield_distribution_event_payload() {
    let mut contract = setup_owner();

    // Seed some deposits so there's yield to distribute
    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(1))
        .attached_deposit(5_000_000_000_000_000_000_000_000);
    testing_env!(ctx.build());
    contract.deposit();

    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(0));
    testing_env!(ctx.build());
    contract.distribute_yield();

    let logs = get_logs();
    let event = find_event(&logs, "vault_yield_distributed");

    assert_eq!(event["standard"], "nep297");
    assert!(
        event["data"][0]["total_yield"].is_string(),
        "total_yield must be a U128 string"
    );
    let _total: u128 = event["data"][0]["total_yield"]
        .as_str()
        .unwrap()
        .parse()
        .expect("total_yield must be parseable as u128");
}

// ── pause / unpause events ────────────────────────────────────────────────────

#[test]
fn test_pause_event_payload() {
    let mut contract = setup_owner();

    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(0));
    testing_env!(ctx.build());
    contract.pause();

    let logs = get_logs();
    let event = find_event(&logs, "vault_paused");
    assert_eq!(event["standard"], "nep297");
    assert!(
        event["data"][0]["paused_by"].is_string(),
        "paused_by field must be present"
    );
}

#[test]
fn test_unpause_event_payload() {
    let mut contract = setup_owner();

    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(0));
    testing_env!(ctx.build());
    contract.pause();
    contract.unpause();

    let logs = get_logs();
    let event = find_event(&logs, "vault_unpaused");
    assert_eq!(event["standard"], "nep297");
}

// ── shares events ─────────────────────────────────────────────────────────────

#[test]
fn test_shares_minted_event_payload() {
    let mut contract = setup_owner();

    let mut ctx = VMContextBuilder::new();
    ctx.predecessor_account_id(accounts(1))
        .attached_deposit(1_000_000_000_000_000_000_000_000);
    testing_env!(ctx.build());
    contract.deposit(); // minting shares is a side-effect of deposit

    let logs = get_logs();
    let event = find_event(&logs, "shares_minted");

    assert!(event["data"][0]["shares"].is_string(), "shares must be U128");
    assert!(
        event["data"][0]["account_id"].is_string(),
        "account_id must be present"
    );
}