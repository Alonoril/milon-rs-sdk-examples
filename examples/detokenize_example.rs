use milon_idl_core::{Address, Detokenize, InvalidOutputType, NamedToken, Token, Tokenizable};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct AccountProfile {
    address: Address,
    enabled: bool,
    display_name: String,
    alias: Option<String>,
    revision: [u8; 4],
    roles: Vec<(u8, String)>,
    balances: BTreeMap<String, u64>,
}

impl Tokenizable for AccountProfile {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        let Token::Struct { name, fields } = token else {
            return Err(InvalidOutputType(
                "expected AccountProfile struct token".to_owned(),
            ));
        };
        if name != "AccountProfile" {
            return Err(InvalidOutputType(format!(
                "expected AccountProfile struct, got {name}"
            )));
        }

        let [
            address,
            enabled,
            display_name,
            alias,
            revision,
            roles,
            balances,
        ] = fields.try_into().map_err(|fields: Vec<NamedToken>| {
            InvalidOutputType(format!(
                "expected 7 AccountProfile fields, got {}",
                fields.len()
            ))
        })?;

        Ok(Self {
            address: Address::from_token(take_field(address, "address")?)?,
            enabled: bool::from_token(take_field(enabled, "enabled")?)?,
            display_name: String::from_token(take_field(display_name, "display_name")?)?,
            alias: Option::<String>::from_token(take_field(alias, "alias")?)?,
            revision: <[u8; 4]>::from_token(take_field(revision, "revision")?)?,
            roles: Vec::<(u8, String)>::from_token(take_field(roles, "roles")?)?,
            balances: BTreeMap::<String, u64>::from_token(take_field(balances, "balances")?)?,
        })
    }

    fn into_token(self) -> Token {
        Token::Struct {
            name: "AccountProfile",
            fields: vec![
                NamedToken {
                    name: "address",
                    value: self.address.into_token(),
                },
                NamedToken {
                    name: "enabled",
                    value: self.enabled.into_token(),
                },
                NamedToken {
                    name: "display_name",
                    value: self.display_name.into_token(),
                },
                NamedToken {
                    name: "alias",
                    value: self.alias.into_token(),
                },
                NamedToken {
                    name: "revision",
                    value: self.revision.into_token(),
                },
                NamedToken {
                    name: "roles",
                    value: self.roles.into_token(),
                },
                NamedToken {
                    name: "balances",
                    value: self.balances.into_token(),
                },
            ],
        }
    }
}

fn take_field(field: NamedToken, expected_name: &'static str) -> Result<Token, InvalidOutputType> {
    if field.name == expected_name {
        Ok(field.value)
    } else {
        Err(InvalidOutputType(format!(
            "expected field {expected_name}, got {}",
            field.name
        )))
    }
}

fn main() {
    let malformed_token = Token::Struct {
        name: "AccountProfile",
        fields: vec![NamedToken {
            name: "enabled",
            value: Token::Bool(true),
        }],
    };

    assert!(AccountProfile::from_token(malformed_token).is_err());

    let profile = AccountProfile {
        address: Address::from_bytes(&[7_u8; 20]).unwrap(),
        enabled: true,
        display_name: "Milon validator".to_owned(),
        alias: Some("validator-7".to_owned()),
        revision: [1, 0, 0, 7],
        roles: vec![(1, "operator".to_owned()), (2, "treasury".to_owned())],
        balances: BTreeMap::from([("MIL".to_owned(), 1_000_000), ("USDM".to_owned(), 42_000)]),
    };
    let token = profile.clone().into_token();

    assert_eq!(AccountProfile::from_token(token.clone()).unwrap(), profile);
    assert_eq!(AccountProfile::from_tokens(vec![token]).unwrap(), profile);
}
