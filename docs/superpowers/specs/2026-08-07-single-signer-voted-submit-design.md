# Single-Signer Voted Submit Design

## Goal

Refactor `app_account_multisig_on_chain::submit` so a normal single-signature
relayer submits and pays for a voted transaction. The multisignature account
remains the vote owner and contributes only the unsigned vote-gate signature.

## Design

- Construct the submit wallet with `LocalWallet::new(relayer_signer)`. The
  relayer is therefore the wallet default account and transaction payer.
- Register the voted multisignature owner in the same wallet. The wallet must
  contain exactly one multisignature account so `WalletFiller` can infer the
  vote owner without adding it to the payer signing plan.
- Build a `SigningPlan` whose payer is the relayer and whose payer
  authorization covers instruction index `0` plus the payer bit. This is
  required because `demo::InitPool.pool` is a `Signer` and is set to the
  relayer address.
- Submit through `send_voted_transaction`. The resulting transaction contains
  two account-signature entries: an unsigned vote-gate entry owned by the
  multisignature account and a signed instruction-plus-payer entry owned by
  the single-signature relayer.
- Remove the seed-255 placeholder default account and the extra
  `register_signer(relayer_signer)` call.

## Error Handling

- Wallet construction and account registration continue to propagate typed
  SDK errors through the example's `Box<dyn Error>` result.
- If no unique multisignature account exists, `WalletFiller` returns the
  existing invalid voted-transaction request error.
- RPC submission and transaction-history lookup errors remain unchanged.

## Verification

- Compile and run the example against the configured RPC endpoint.
- Require a successful transaction receipt (`state: 1`, `error: None`).
- Run the SDK workspace tests, including the voted relayer signing tests.
