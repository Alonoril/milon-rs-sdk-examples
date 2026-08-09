use milon_idl_core::{Address, Detokenize, NamedToken, Token, Tokenizable as _};
use milon_idl_macro::Tokenizable;
use milon_primitives::AnySigner;
use std::collections::BTreeMap;

mod tokenizable_account_profile;

#[derive(Clone, Debug, PartialEq, Eq, Tokenizable)]
struct AccountProfile {
    address: Address,
    enabled: bool,
    display_name: String,
    any_signer: AnySigner,
    alias: Option<String>,
    revision: [u8; 4],
    roles: Vec<(u8, String)>,
    balances: BTreeMap<String, u64>,
}

fn main() {
    let malformed_token = Token::Struct {
        name: "AccountProfile",
        fields: vec![NamedToken {
            name: "enabled",
            value: Token::Bool(true),
        }],
    };
    println!("{}", malformed_token);

    assert!(AccountProfile::from_token(malformed_token).is_err());

    let profile = AccountProfile {
        address: Address::from_bytes(&[7_u8; 20]).unwrap(),
        enabled: true,
        display_name: "Milon validator".to_owned(),
        any_signer: AnySigner::new(Address::from_bs58("2oiqKaWR44836m1QRKDhQc5MSLMM").unwrap()),
        alias: Some("validator-7".to_owned()),
        revision: [1, 0, 0, 7],
        roles: vec![(1, "operator".to_owned()), (2, "treasury".to_owned())],
        balances: BTreeMap::from([("MIL".to_owned(), 1_000_000), ("USDM".to_owned(), 42_000)]),
    };
    let token = profile.clone().into_token();
    println!(">>>profile token: {:#?}", token);

    let res = AccountProfile::from_token(token.clone()).unwrap();
    println!("acct>>>{:?}", res);

    assert_eq!(res, profile);
    assert_eq!(AccountProfile::from_tokens(vec![token]).unwrap(), profile);
}
