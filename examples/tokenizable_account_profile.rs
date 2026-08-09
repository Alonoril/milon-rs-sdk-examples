use crate::AccountProfile;
use milon_crypto::Address;
use milon_primitives::AnySigner;
use std::collections::BTreeMap;

impl ::milon_idl_core::Tokenizable for AccountProfile {
    fn from_token(
        token: ::milon_idl_core::Token,
    ) -> ::core::result::Result<Self, ::milon_idl_core::InvalidOutputType> {
        let ::milon_idl_core::Token::Struct { name, fields } = token else {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected {} struct token",
                "AccountProfile"
            )));
        };
        if name != "AccountProfile" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected {} struct, got {name}",
                "AccountProfile"
            )));
        }
        let fields: [::milon_idl_core::NamedToken; 8usize] = fields.try_into().map_err(
            |fields: ::std::vec::Vec<::milon_idl_core::NamedToken>| {
                ::milon_idl_core::InvalidOutputType(format!(
                    "expected {} {} fields, got {}",
                    8usize,
                    "AccountProfile",
                    fields.len()
                ))
            },
        )?;
        let [
            address,
            enabled,
            display_name,
            any_signer,
            alias,
            revision,
            roles,
            balances,
        ] = fields;
        if address.name != "address" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "address", address.name,
            )));
        }
        if enabled.name != "enabled" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "enabled", enabled.name,
            )));
        }
        if display_name.name != "display_name" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "display_name", display_name.name,
            )));
        }
        if any_signer.name != "any_signer" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "any_signer", any_signer.name,
            )));
        }
        if alias.name != "alias" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "alias", alias.name,
            )));
        }
        if revision.name != "revision" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "revision", revision.name,
            )));
        }
        if roles.name != "roles" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "roles", roles.name,
            )));
        }
        if balances.name != "balances" {
            return Err(::milon_idl_core::InvalidOutputType(format!(
                "expected field {}, got {}",
                "balances", balances.name,
            )));
        }
        Ok(Self {
            address: <Address as ::milon_idl_core::Tokenizable>::from_token(address.value)?,
            enabled: <bool as ::milon_idl_core::Tokenizable>::from_token(enabled.value)?,
            display_name: <String as ::milon_idl_core::Tokenizable>::from_token(
                display_name.value,
            )?,
            any_signer: <AnySigner as ::milon_idl_core::Tokenizable>::from_token(any_signer.value)?,
            alias: <Option<String> as ::milon_idl_core::Tokenizable>::from_token(alias.value)?,
            revision: <[u8; 4] as ::milon_idl_core::Tokenizable>::from_token(revision.value)?,
            roles: <Vec<(u8, String)> as ::milon_idl_core::Tokenizable>::from_token(roles.value)?,
            balances: <BTreeMap<String, u64> as ::milon_idl_core::Tokenizable>::from_token(
                balances.value,
            )?,
        })
    }

    fn into_token(self) -> ::milon_idl_core::Token {
        ::milon_idl_core::Token::Struct {
            name: "AccountProfile",
            fields: ::std::vec![
                ::milon_idl_core::NamedToken {
                    name: "address",
                    value: <Address as ::milon_idl_core::Tokenizable>::into_token(self.address,),
                },
                ::milon_idl_core::NamedToken {
                    name: "enabled",
                    value: <bool as ::milon_idl_core::Tokenizable>::into_token(self.enabled,),
                },
                ::milon_idl_core::NamedToken {
                    name: "display_name",
                    value: <String as ::milon_idl_core::Tokenizable>::into_token(self.display_name,),
                },
                ::milon_idl_core::NamedToken {
                    name: "any_signer",
                    value: <AnySigner as ::milon_idl_core::Tokenizable>::into_token(
                        self.any_signer,
                    ),
                },
                ::milon_idl_core::NamedToken {
                    name: "alias",
                    value: <Option<String> as ::milon_idl_core::Tokenizable>::into_token(
                        self.alias,
                    ),
                },
                ::milon_idl_core::NamedToken {
                    name: "revision",
                    value: <[u8; 4] as ::milon_idl_core::Tokenizable>::into_token(self.revision,),
                },
                ::milon_idl_core::NamedToken {
                    name: "roles",
                    value: <Vec<(u8, String)> as ::milon_idl_core::Tokenizable>::into_token(
                        self.roles,
                    ),
                },
                ::milon_idl_core::NamedToken {
                    name: "balances",
                    value: <BTreeMap<String, u64> as ::milon_idl_core::Tokenizable>::into_token(
                        self.balances,
                    ),
                },
            ],
        }
    }
}

fn main() {}
