# Solendrix

**Solendrix** is a Rust-based decentralized lending protocol built for the Solana blockchain.  
The project is built completely in Rust using the [Pinocchio](https://github.com/anza-xyz/pinocchio) framework.
It allows users to **deposit one token as collateral and borrow another token**, leveraging program-derived accounts (PDAs) and secure SPL token vaults.

---

## Features

- **Market Initialization**  
  Admins can create a market PDA that manages token vaults and lending parameters.

- **User Accounts**  
  Each user gets a PDA-based account to track deposits, borrows, and collateralization.

- **Deposit Collateral**  
  Users deposit **Token A** into the market vault to increase their collateral balance.

- **Borrow Against Collateral**  
  Users can borrow **Token B** against their deposited Token A (subject to collateralization rules).

- **Withdraw Collateral**  
  Users can safely withdraw deposited collateral, ensuring they remain above liquidation thresholds.

- **Secure SPL Token Vaults**  
  Vaults are owned by the market PDA, preventing unauthorized withdrawals.

---

Prerequisites

Rust + Cargo

Solana CLI + local validator

Optional: LiteSVM for lightweight transaction simulation

## Build
```bash
cargo build-bpf
```

##Test
```bash
cargo test
```

## Future Improvements

Interest rate models for borrowed tokens

Liquidation mechanics for undercollateralized users

Dynamic collateral ratios per market

Frontend dApp integration (React + Anchor client)