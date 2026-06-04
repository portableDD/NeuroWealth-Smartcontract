# Security Model

This document describes the security architecture, trust model, and threat model for the NeuroWealth Vault contract.

## Trust Model

The NeuroWealth Vault implements a partitioned trust model with three distinct roles:

### Owner

The contract owner has the following permissions:
- **Pause/Unpause**: Can halt all deposits and withdrawals during emergencies
- **Set TVL Cap**: Can limit total deposits to manage risk exposure
- **Set User Deposit Cap**: Can limit per-user exposure
- **Update Agent**: Can change the authorized AI agent address
- **Upgrade Contract**: Can upgrade contract code (Phase 2)

The owner **CANNOT**:
- Access user funds directly
- Withdraw funds from user accounts
- Modify user balances

### AI Agent

The authorized AI agent has the following permissions:
- **Rebalance**: Can call `rebalance()` to signal strategy changes and move funds between protocols
- **Update Total Assets**: Can report yield accrual or strategy losses
- **Emergency Pause**: Can trigger an immediate emergency pause if anomalies are detected
- **Read Access**: Can read all vault state to make yield decisions

The agent **CANNOT**:
- Withdraw user funds directly to itself
- Change vault configuration (caps, pools)
- Access USDC tokens directly outside of protocol interactions
- Modify user balances without valid asset reporting

### Users

Regular users have the following permissions:
- **Deposit**: Can deposit USDC into the vault
- **Withdraw**: Can withdraw their own USDC at any time
- **Read**: Can query their balance and vault state

Users **CANNOT**:
- Access other users' funds
- Manipulate vault configuration
- Call agent-only or owner-only functions

## Withdrawal Guarantees

### Automated Liquidity Management

The vault automatically manages liquidity between idle USDC (held in the contract) and deployed assets (e.g., in Blend protocol):
1. **Idle Withdrawals**: If the vault holds sufficient idle USDC, withdrawals are processed immediately.
2. **Protocol Withdrawals**: If idle USDC is insufficient, the vault automatically attempts to withdraw the required amount from the active protocol (e.g., Blend).
3. **Partial Withdrawals**: If the protocol has insufficient liquidity (e.g., high utilization), the user receives all available USDC and **retains their remaining shares** in the vault. This ensures users are not forced into unfavorable liquidations during protocol-wide liquidity crunches.

### Withdrawal Priority

Users can withdraw their USDC at any time without:
- Lock-up periods
- Withdrawal fees
- Approval requirements beyond their signature

## Risk Analysis

### 1. External Protocol Risk (Blend)

Integration with protocols like Blend introduces systemic risk:
- **Liquidity Risk**: If Blend utilization is 100%, the vault cannot pull funds immediately. Users will experience partial withdrawals until liquidity returns to the protocol.
- **Protocol Failure**: A bug or exploit in Blend could result in loss of deployed assets.

### 2. Asset Reporting Risk

The `update_total_assets` function used by the AI agent has built-in guardrails:
- **Solvency Check**: The agent cannot inflate total assets beyond the combined balance of idle USDC and funds actually deployed to external protocols.
- **Decrease Bounding**: Reporting a loss is capped (default 10% per call) to prevent sudden, massive devaluations from a single malicious or erroneous call.

### 3. Upgrade Risks

The contract owner can upgrade the contract code. This introduces:
- **Single Point of Failure**: The owner key is a high-value target.
- **Mitigation**: Use multi-sig for the owner and timelocks for code upgrades.

## Access Control Summary

| Function | Owner | Agent | User | Anyone |
|----------|-------|-------|------|--------|
| set_agent | ✅ | - | - | - |
| update_total_assets | - | ✅ | - | - |
| deposit | - | - | ✅ | - |
| withdraw | - | - | ✅ | - |
| rebalance | - | ✅ | - | - |
| pause | ✅ | - | - | - |
| emergency_pause | - | ✅ | - | - |
| unpause | ✅ | - | - | - |
| set_caps | ✅ | - | - | - |
| upgrade | ✅ | - | - | - |

## Security Best Practices Implemented

1. **Checks-Effects-Interactions Pattern**: All state updates happen before external calls
2. **Auth on Withdrawals**: `require_auth()` ensures users can only access their own funds
3. **Minimum Deposits**: Prevents dust attacks
4. **Deposit Caps**: Limits exposure per user
5. **TVL Caps**: Limits total exposure
6. **Pausable**: Emergency stop functionality

## Audit & Mainnet Deployment Checklist

Before any mainnet deployment, you must refer to and complete the formal [Mainnet Deployment Checklist](docs/MAINNET_CHECKLIST.md).

Additionally, ensure:

- [ ] All functions have documented panic conditions
- [ ] All state changes emit events
- [ ] Access control verified for each function
- [ ] Upgrade mechanism tested on testnet
- [ ] Pause/unpause tested
- [ ] Withdrawal flow tested with edge cases
- [ ] Maximum deposit limits enforced
- [ ] TVL cap enforced
- [ ] Integration with USDC token tested
- [ ] Integration with Blend protocol tested (Phase 2)
