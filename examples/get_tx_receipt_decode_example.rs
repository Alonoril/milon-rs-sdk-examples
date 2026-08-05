use milon_provider::Provider;
use milon_client::{self as sdk, primitives::TxHash};
use only_sdk_examples::{DemoRpc, decode_print::print_transaction_history};
use std::{env, error::Error};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";
const DEFAULT_TX_HASH_BS58: &str = "BLuTczC8igRgHiauGagUYYK85A9BsP25Z4SWxcEHKEaX";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_HTTP_RPC_URL.to_owned());
    let rpc = DemoRpc::connect(&rpc_url)?;

    let chain_state = rpc.provider.get_chain_head().await?;
    println!("chain_head: {:?}", chain_state);

    get_transaction_history(&rpc).await?;

    Ok(())
}

async fn get_transaction_history(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let tx_hash = load_tx_hash()?;

    // let transport = HttpInvokeTransport::new(Url::parse(rpc_url)?);
    // let client = RpcClient::builder().transport(transport, false);
    // let raw: RawBytes = client
    //     .request(
    //         SdkMethod::GetTransactionByHash,
    //         TxHashParam::new(tx_hash.to_bytes()),
    //     )
    //     .await?;
    let raw: Vec<u8> = rpc.provider.get_transaction_by_hash(tx_hash).await?;
    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);

    Ok(())
}

fn load_tx_hash() -> Result<TxHash, Box<dyn Error>> {
    if let Ok(value) = env::var("MILON_TX_HASH_BS58") {
        return Ok(TxHash::from_bs58(value.trim())?);
    }
    if let Ok(value) = env::var("MILON_TX_HASH_HEX") {
        return parse_tx_hash_hex(value.trim());
    }
    Ok(TxHash::from_bs58(DEFAULT_TX_HASH_BS58)?)
}

fn parse_tx_hash_hex(value: &str) -> Result<TxHash, Box<dyn Error>> {
    let bytes = hex::decode(value)?;
    let hash: [u8; 32] = bytes.try_into().map_err(|bytes: Vec<u8>| {
        format!("expected 32-byte tx hash hex, got {} bytes", bytes.len())
    })?;
    Ok(TxHash::new(hash))
}

#[cfg(test)]
mod tests {
    use milon_idl_core::{NamedToken, Token};
    use milon_client::demo;
    use only_sdk_examples::decode_print::decode_inline_resource;

    #[test]
    fn inline_resource_decodes_with_sdk_type_tag() {
        let raw = postcard::to_allocvec(&demo::Label {
            text: "resource".to_owned(),
        })
        .expect("encode resource");

        let decoded = decode_inline_resource(demo::Label::TYPE_TAG, &raw).expect("decode resource");

        assert_eq!(decoded.name, "Label");
        assert_eq!(decoded.token, Token::Struct {
            name: "Label",
            fields: vec![NamedToken {
                name: "text",
                value: Token::String("resource".to_owned()),
            }],
        });
    }
}
