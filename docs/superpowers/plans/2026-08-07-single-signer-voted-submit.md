# Single-Signer Voted Submit Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor `app_account_multisig_on_chain::submit` so one ordinary relayer signer is the wallet default account, transaction payer, instruction signer, and submitter while the registered multisig account is used only as the vote owner.

**Architecture:** Extract wallet construction into a small synchronous helper that creates `LocalWallet` from the relayer signer and then registers the voted multisig owner. Keep `submit` responsible for faucet funding, constructing the payer authorization for instruction `0`, sending the voted transaction, and printing the receipt. `WalletFiller` infers the sole registered multisig account as the vote owner.

**Tech Stack:** Rust 2024, `milon_local_wallet`, `milon_provider`, `milon_client`, Tokio, live Milon RPC.

## Global Constraints

- Do not write files under `/Users/egal/snqu_ws/milon-labs/Milon/docs`.
- Preserve tabs and Rust 2024 formatting through the repository formatter.
- Keep the relayer as a normal single-signature account; do not convert it into a multisignature account.
- The resulting transaction must contain a multisig vote-gate signature and a distinct signed relayer payer/instruction signature.
- Preserve unrelated working-tree changes and `.DS_Store` without staging them.

---

### Task 1: Refactor and verify single-signer voted submit

**Files:**
- Modify: `examples/app_account_multisig_on_chain.rs`
- Test: `examples/app_account_multisig_on_chain.rs` inline `tests` module

**Interfaces:**
- Consumes: `LocalWallet::new(LocalSigner)`, `LocalWallet::register_multisig(Address, u8, Vec<MultisigSlot>)`, `LocalWallet::sole_multisig_account()`, `SigningPlan::new(Address)`, and `AccountAuthorization::with_payer()`.
- Produces: `fn build_submit_wallet(owner: Address) -> Result<(LocalWallet, Address), Box<dyn Error>>` and the existing `async fn submit(...)` using that helper.

- [ ] **Step 1: Add a failing test for the submit wallet identity split**

Add this import and test to the existing inline `tests` module:

```rust
use super::{VoteIntent, build_submit_wallet};
use milon_local_wallet::Signer;

#[test]
fn submit_wallet_uses_single_signer_relayer_and_separate_vote_owner() {
	let owner = super::local_ed25519_signer(super::OWNER_SIGNER_SEED)
		.unwrap()
		.address();
	let (wallet, payer) = build_submit_wallet(owner).unwrap();

	assert_eq!(wallet.default_account(), payer);
	assert_ne!(payer, owner);
	assert_eq!(wallet.sole_multisig_account(), Some(owner));
}
```

- [ ] **Step 2: Run the test and confirm the helper is missing**

Run:

```bash
cargo test --example app_account_multisig_on_chain submit_wallet_uses_single_signer_relayer_and_separate_vote_owner -- --exact
```

Expected: compilation fails because `build_submit_wallet` is not defined.

- [ ] **Step 3: Extract single-signer wallet construction**

Add the helper immediately before `submit`:

```rust
fn build_submit_wallet(owner: Address) -> Result<(LocalWallet, Address), Box<dyn Error>> {
	let account_signer = local_ed25519_signer(OWNER_SIGNER_SEED)?;
	let idx2_vote_signer = local_ed25519_signer(IDX2_VOTE_SIGNER_SEED)?;
	let idx3_vote_signer = local_ed25519_signer(IDX3_VOTE_SIGNER_SEED)?;
	let relayer_signer = local_ed25519_signer(RELAYER_SIGNER_SEED)?;
	let payer = relayer_signer.address();
	let slots = vec![
		MultisigSlot::new(0, account_signer),
		MultisigSlot::with_weight(2, 2, idx2_vote_signer),
		MultisigSlot::with_weight(3, 3, idx3_vote_signer),
	];
	let mut wallet = LocalWallet::new(relayer_signer);
	wallet.register_multisig(owner, 5, slots)?;
	Ok((wallet, payer))
}
```

Replace the signer and wallet setup at the start of `submit` with:

```rust
let (wallet, payer) = build_submit_wallet(owner)?;
let provider = rpc
	.provider
	.with_wallet_filler(WalletFiller::new(wallet));
```

Keep the existing payer authorization exactly as:

```rust
let plan =
	SigningPlan::new(payer).authorize(AccountAuthorization::new(payer, vec![0]).with_payer());
```

Remove the obsolete commented duplicate `submit` implementation at the end of the file; it describes the seed-255 placeholder design being replaced.

- [ ] **Step 4: Format and run the focused tests**

Run:

```bash
cargo fmt --all
cargo fmt --all -- --check
cargo test --example app_account_multisig_on_chain
```

Expected: formatting succeeds and both inline example tests pass.

- [ ] **Step 5: Run the real voted submit against the configured RPC**

Run:

```bash
cargo run --example app_account_multisig_on_chain
```

Expected output includes all of:

```text
submit tx_hash:
state: 1 (success)
error: None
>>>>>>multisig_on_chain res: Ok(())
```

- [ ] **Step 6: Commit the refactor without staging unrelated files**

```bash
git add examples/app_account_multisig_on_chain.rs
git commit -m "refactor: submit voted transaction with single signer"
```
