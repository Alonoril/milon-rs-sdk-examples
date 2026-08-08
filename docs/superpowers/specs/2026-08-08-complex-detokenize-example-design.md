# Complex Detokenize Example Design

## Goal

Provide a standalone runnable example showing how an application-defined complex struct implements `milon_idl_core::Tokenizable` and round-trips through `Token`.

## Structure

The example defines `AccountProfile` with a fixed account address, enabled flag, display name, optional alias, fixed `[u8; 4]` revision, ordered role entries (`Vec<(u8, String)>`), and ordered balances (`BTreeMap<String, u64>`). These fields exercise primitive, crypto, option, array, vector, tuple, and map conversions.

`Tokenizable::into_token` produces `Token::Struct { name: "AccountProfile", fields }` with a stable field order. `from_token` requires exactly that struct name and field sequence, then delegates each nested conversion to the SDK's generic `Tokenizable` implementations. Shape, type, name, and field-count mismatches return `InvalidOutputType`; the example does not panic for malformed dynamic tokens.

## Demonstration

`main` constructs an `AccountProfile`, converts it to `Token`, converts it back with `AccountProfile::from_token`, and asserts equality. It also demonstrates `Detokenize::from_tokens` with the same struct as a single output.

## Scope

Only `only-sdk-examples/examples/detokenize_example.rs` changes. No SDK API or macro changes are required.
