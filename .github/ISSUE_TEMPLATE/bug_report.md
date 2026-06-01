---
name: Bug Report
about: Report a defect in the smart contract or related systems
title: "Bug: "
labels: ["bug"]
assignees: []
---
## Description

Provide a clear and concise description of the bug. What is the issue?
## Soroban Network

Which Soroban network did you encounter this bug on?

- [ ] Devnet (local development network)
- [ ] Testnet (public test network)
- [ ] Mainnet (production network)

**Why this matters**: Network-specific bugs help us identify environment-related issues.

## Contract Information

### Contract ID

Provide the contract ID where the bug occurs (if applicable).

Example format: `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4`

**How to find it**:
- Check deployment logs
- Look it up on [StellarExpert](https://stellar.expert/)
- Use `stellar contract info --id <CONTRACT_ID>`

### Contract Function

Which contract function is affected? (e.g., `deposit`, `withdraw`, `rebalance`)

### Contract Parameters

What parameters or arguments were used when calling the function?

## Steps to Reproduce

Provide step-by-step instructions to reproduce the bug:

1. 
2. 
3. 

**Example**:
```
1. Run `cargo test --verbose`
2. Execute `stellar contract invoke --id <CONTRACT_ID> -- deposit 1000`
3. Observe the error
```

## Expected Behavior

Describe what you expected to happen.

## Actual Behavior

Describe what actually happened instead.

## Test Command Output

Provide the output of the failing test or command. This is essential for debugging.

### Command Used

What command did you run?

Examples:
- `cargo test --verbose`
- `stellar contract invoke --id <CONTRACT_ID> -- <function> <args>`
- `npm test`

### Full Output

Paste the complete output (including error messages and stack traces):

```
[Paste output here]
```

**Tip**: Use code blocks (triple backticks) for formatting.

## Environment Details

Provide your environment information to help identify version-specific bugs.

### Versions

- Rust version: `rustc --version`
- Stellar CLI version: `stellar --version`
- Node.js version: `node --version` (if applicable)
- Cargo version: `cargo --version`

### System Information

- Operating system and version: (e.g., macOS 14.0, Ubuntu 22.04, Windows 11)
- Architecture: (e.g., x86_64, ARM64)

### Soroban Network

- Network: (devnet/testnet/mainnet)
- RPC endpoint: (if using custom endpoint)

## Error Message

If you received an error message, please provide it here.

**Note**: Review the [Error Message Style Guide](ERROR_STYLE_GUIDE.md) to understand error categories.

```
[Paste exact error message here]
```

## Verification Checklist

Before submitting, please verify:

- [ ] I have run `cargo fmt` and `cargo clippy` on my changes
- [ ] I have run the full test suite (`cargo test`)
- [ ] I have provided all required information above
- [ ] I have reviewed the [CONTRIBUTING.md](CONTRIBUTING.md) guide
- [ ] This is not a security issue (see [SECURITY.md](SECURITY.md))

**Note**: Incomplete issues may be closed and asked to be resubmitted.

## Additional Context

Add any other context about the problem here (screenshots, links, related discussions, etc.).

## Transaction Hash

If this is an on-chain issue, provide the transaction hash:

```
[Paste transaction hash here]
```

**How to find it**:
- Check [StellarExpert](https://stellar.expert/) for transaction details
- Use `stellar transaction info --hash <TRANSACTION_HASH>`

## Security Notice

⚠️ **If this is a security issue**, please report it via [SECURITY.md](SECURITY.md) instead of creating a public issue. Do not disclose security vulnerabilities publicly.

- [ ] This is a security issue (report via SECURITY.md instead)
