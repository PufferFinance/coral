use std::path;

use coral_lib::{
    error::{AppError, AppErrorKind, AppResult},
    strip_0x_prefix,
};
use ethers::prelude::*;
use ethers::types::Address;
use ethers::utils::hex;

use coral_lib::utils;

use crate::{
    commands::validator::keygen::BlsKeygenOutput, Permit, ValidatorKeyData, ValidatorTicket,
};

use crate::PufferProtocol;

pub async fn generate_register_calldata(
    rpc_url: &str,
    puffer_protocol_address: &str,
    validator_ticket_address: &str,
    module_name: &str,
    input_file: &path::Path,
) -> AppResult<i32> {
    let puffer_protocol_address_h160: Address = puffer_protocol_address.parse().map_err(|_| {
        AppError::new(
            AppErrorKind::DecodeError,
            format!(
                "Invalid Puffer Protocol address: '{}'",
                puffer_protocol_address
            ),
        )
    })?;

    let validator_ticket_address_h160: Address =
        validator_ticket_address.parse().map_err(|_| {
            AppError::new(
                AppErrorKind::DecodeError,
                format!(
                    "Invalid Validator Ticket address: '{}'",
                    validator_ticket_address
                ),
            )
        })?;

    let module_name = utils::parse::parse_module_name(module_name)?;

    let provider = utils::ethereum::get_provider(rpc_url)?;

    let wallet = utils::wallet::generate_random_wallet();

    let chain_id = utils::ethereum::get_chain_id(&provider).await?;
    let client = utils::ethereum::get_client(provider.clone(), wallet.clone(), chain_id.as_u64());

    let content = std::fs::read_to_string(input_file)?;
    let keygen_data: BlsKeygenOutput = serde_json::from_str(&content).unwrap();

    println!("Generating calldata...");

    let intel_report = keygen_data.intel_report.as_bytes();
    let intel_sig = keygen_data.intel_sig.as_bytes();
    let intel_x509 = keygen_data.intel_x509.as_bytes();

    let rave_evidence =
        utils::abi::rave_evidence::to_calldata(intel_sig, intel_report, intel_x509)?;

    let bls_pub_key_set =
        hex::decode(strip_0x_prefix(&keygen_data.bls_pub_key_set)).map_err(|err| {
            let error_msg = format!("Failed to decode RAVE evidence: {err}");
            AppError::new(AppErrorKind::DecodeError, error_msg)
        })?;

    let bls_pub_key = hex::decode(strip_0x_prefix(&keygen_data.bls_pub_key)).map_err(|err| {
        let error_msg = format!("Failed to decode BLS Pub Key: {err}");
        AppError::new(AppErrorKind::DecodeError, error_msg)
    })?;

    let signature = hex::decode(strip_0x_prefix(&keygen_data.signature)).map_err(|err| {
        let error_msg = format!("Failed to decode signature: {err}");
        AppError::new(AppErrorKind::DecodeError, error_msg)
    })?;

    let bls_encrypted_priv_key_shares: Vec<Bytes> = keygen_data
        .bls_enc_priv_key_shares
        .iter()
        .map(|keyshare| {
            let keyshare: &str = strip_0x_prefix(keyshare);
            hex::decode(keyshare).unwrap().into()
        })
        .collect();

    let deposit_data_root: [u8; 32] = hex::decode(strip_0x_prefix(&keygen_data.deposit_data_root))
        .unwrap()
        .try_into()
        .unwrap();

    let validator_data = ValidatorKeyData {
        bls_pub_key: bls_pub_key.into(),
        signature: signature.into(),
        deposit_data_root,
        bls_encrypted_priv_key_shares,
        bls_pub_key_set: bls_pub_key_set.into(),
        rave_evidence: rave_evidence.into(),
    };

    let _validator_ticket_contract: ValidatorTicket<_> =
        ValidatorTicket::new(validator_ticket_address_h160, client.clone());

    let puf_eth_deposit_permit = Permit {
        deadline: U256::zero(),
        amount: U256::zero(),
        v: 0,
        r: [0; 32],
        s: [0; 32],
    };

    let vt_deposit_permit = Permit {
        deadline: U256::zero(),
        amount: U256::zero(),
        v: 0,
        r: [0; 32],
        s: [0; 32],
    };

    let puffer_protocol_contract: PufferProtocol<_> =
        PufferProtocol::new(puffer_protocol_address_h160, client.clone());

    let calldata = puffer_protocol_contract
        .register_validator_key(
            validator_data,
            module_name,
            puf_eth_deposit_permit,
            vt_deposit_permit,
        )
        .calldata()
        .ok_or_else(|| {
            let error_msg = "Failed to generate calldata";
            tracing::error!("{error_msg}");
            AppError::new(AppErrorKind::ContractCallError, error_msg.to_string())
        })?;
    println!("{calldata}");

    Ok(0)
}
