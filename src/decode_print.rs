use milon_primitives::{AccessRecord, TxHistory};
use milon_client::{self as sdk, get_app_err, AccessValue};

pub type DecodedReceipt = sdk::Receipt<sdk::idl_core::DecodedResource, sdk::idl_core::DecodedEvent>;

pub fn print_decoded_instructions(instructions: &[sdk::PackedInstruction]) {
    println!("instructions:");
    for (index, instruction) in instructions.iter().enumerate() {
        match sdk::decode_instruction(instruction) {
            Ok(decoded) => println!("  instruction[{index}]: {decoded}"),
            Err(error) => println!(
                "  instruction[{index}]: decode failed ({error}), raw={}",
                hex::encode(instruction)
            ),
        }
    }
}

pub fn print_simulate_receipt(receipt: &DecodedReceipt) {
    println!("simulate receipt:");
    println!("  tx_id: {}", receipt.tx_id);
    println!("  tx_hash: {}", receipt.tx_hash);
    println!(
        "  state: {} ({})",
        receipt.state,
        tx_state_label(receipt.state)
    );
    println!("  access_count: {}", receipt.access.len());
    for (index, record) in receipt.access.iter().enumerate() {
        println!("  access[{index}].resource_id: {:?}", record.resource_id);
        print_access_value("first_snapshot", index, record.first_snapshot.as_ref());
        print_access_value("last_written", index, Some(&record.last_written));
    }
    println!("  event_count: {}", receipt.events.len());
    for (index, event) in receipt.events.iter().enumerate() {
        println!("  event[{index}]: {event:?}");
    }
    println!("  error: {:?}", receipt.error);
}

// fn tx_state_label(state: u8) -> &'static str {
//     match state {
//         0 => "pending",
//         1 => "success",
//         2 => "failed",
//         _ => "unknown",
//     }
// }

fn print_access_value(
    field: &str,
    index: usize,
    value: Option<&AccessValue<sdk::idl_core::DecodedResource>>,
) {
    match value {
        Some(AccessValue::Inline(resource)) => {
            println!("  access[{index}].{field}: inline {resource:?}");
        },
        Some(AccessValue::External(hash)) => {
            println!("  access[{index}].{field}: external {hash}");
        },
        None => println!("  access[{index}].{field}: none"),
    }
}

pub fn print_transaction_history(
    history: &TxHistory<sdk::idl_core::DecodedResource, sdk::idl_core::DecodedEvent>,
) {
    println!("transaction history:");
    println!("  stamp: {}", history.stamp);
    println!("  payer_signature_index: {:?}", history.payer);
    println!("  signature_count: {}", history.signatures.len());
    println!("  tx_id: {}", hex::encode(history.receipt.tx_id));
    println!("  tx_hash: {}", history.receipt.tx_hash);
    println!(
        "  state: {} ({})",
        history.receipt.state,
        tx_state_label(history.receipt.state)
    );
    println!("  gas_charged: {}", history.receipt.gas_charged);
    println!("  error: {:?}", history.receipt.error);
    if let Some(err) = history.receipt.error {
        let res = get_app_err(err).expect("failed to get app error from idl");
        println!("  error reason: {:?}", res);
    }

    print_decoded_instructions2(&history.instructions);
    print_decoded_access_changes(&history.receipt.access);
    print_decoded_events(&history.receipt.events);
}

fn print_decoded_instructions2(instructions: &[sdk::PackedInstruction]) {
    println!("  instruction_count: {}", instructions.len());
    for (index, instruction) in instructions.iter().enumerate() {
        match sdk::decode_instruction(instruction) {
            Ok(decoded) => println!("    instruction[{index}]: {decoded}"),
            Err(error) => println!(
                "    instruction[{index}]: decode failed ({error}), raw={}",
                hex::encode(instruction)
            ),
        }
    }
}

pub fn print_decoded_access_changes(
    access_changes: &[AccessRecord<sdk::idl_core::DecodedResource>],
) {
    println!("  access_resource_count: {}", access_changes.len());
    for (index, record) in access_changes.iter().enumerate() {
        println!("    access[{index}].resource_id: {:?}", record.resource_id);
        print_access_value2(index, "first_snapshot", record.first_snapshot.as_ref());
        print_access_value2(index, "last_written", Some(&record.last_written));
    }
}

fn print_access_value2(
    index: usize,
    field: &str,
    value: Option<&AccessValue<sdk::idl_core::DecodedResource>>,
) {
    match value {
        Some(AccessValue::Inline(resource)) => {
            println!("      access[{index}].{field}: inline {resource:?}");
        },
        Some(AccessValue::External(hash)) => {
            println!("      access[{index}].{field}: external {hash}");
        },
        None => println!("      access[{index}].{field}: none"),
    }
}

pub fn print_decoded_events(events: &[sdk::idl_core::DecodedEvent]) {
    println!("  event_count: {}", events.len());
    for (index, event) in events.iter().enumerate() {
        println!("    event[{index}]: {event:?}");
    }
}

pub fn decode_inline_resource(
    type_tag: u64,
    bytes: &[u8],
) -> sdk::idl_core::Result<sdk::idl_core::DecodedResource> {
    sdk::decode_resource(type_tag, bytes)
}

pub fn tx_state_label(state: u8) -> &'static str {
    match state {
        0 => "pending",
        1 => "success",
        2 => "failed",
        _ => "unknown",
    }
}
