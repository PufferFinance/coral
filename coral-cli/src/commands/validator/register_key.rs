use std::path;

use coral_lib::{
    error::{AppError, AppErrorKind, AppResult},
    strip_0x_prefix,
};
use ethers::{signers::LocalWallet, types::Address};

use ethers::prelude::*;
use ethers::utils::hex::{self, ToHex};

use coral_lib::utils;

use crate::{
    commands::validator::keygen::RegisterValidatorOutput, Permit, PufferOracle, ValidatorKeyData,
    ValidatorTicket,
};

use crate::PufferProtocol;

pub async fn register_validator_key(
    private_key: &str,
    rpc_url: &str,
    puffer_oracle_address: &str,
    puffer_protocol_address: &str,
    validator_ticket_address: &str,
    module_name: &str,
    number_of_days: u64,
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

    let puffer_oracle_address_h160: Address = puffer_oracle_address.parse().map_err(|_| {
        AppError::new(
            AppErrorKind::DecodeError,
            format!("Invalid Puffer Oracle address: '{}'", puffer_oracle_address),
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

    println!("Parsing private key...");
    let priv_key_bytes = hex::decode(strip_0x_prefix(private_key)).unwrap();
    let wallet = LocalWallet::from_bytes(&priv_key_bytes).unwrap();

    let chain_id = utils::ethereum::get_chain_id(&provider).await?;
    let client = utils::ethereum::get_client(provider.clone(), wallet.clone(), chain_id.as_u64());

    let content = std::fs::read_to_string(input_file)?;
    let keygen_data: RegisterValidatorOutput = serde_json::from_str(&content).unwrap();

    println!("Generating calldata...");

    let enclave_enabled = !keygen_data.intel_report.is_empty();

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
        deposit_data_root: deposit_data_root,
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

    let puffer_oracle_contract: PufferOracle<_> =
        PufferOracle::new(puffer_oracle_address_h160, client.clone());

    let vt_price: U256 = puffer_oracle_contract
        .get_validator_ticket_price()
        .await
        .map_err(|err| {
            let error_msg = format!("Failed to fetch vt price: {err}");
            AppError::new(AppErrorKind::DecodeError, error_msg)
        })?;

    let puffer_protocol_contract: PufferProtocol<_> =
        PufferProtocol::new(puffer_protocol_address_h160, client.clone());

    let eth_1 = U256::from(1).saturating_mul(U256::exp10(18));
    let eth_2 = U256::from(2).saturating_mul(U256::exp10(18));

    let total_vt_price = vt_price.saturating_mul(U256::from(number_of_days));

    println!("Registering validator to smart contract...");
    let function_call = puffer_protocol_contract
        .register_validator_key(
            validator_data,
            module_name,
            U256::from(number_of_days),
            puf_eth_deposit_permit,
            vt_deposit_permit,
        )
        .value(if enclave_enabled {
            eth_1.saturating_add(total_vt_price)
        } else {
            eth_2.saturating_add(total_vt_price)
        });
    let res = function_call.send().await;

    match res {
        Ok(pending_tx) => {
            let tx = pending_tx
                .await
                .map_err(|err| {
                    tracing::error!("Failed to wait for pending transaction");
                    tracing::error!("{err}");
                    AppError::new(
                        AppErrorKind::ContractCallError,
                        "Failed to wait for pending transaction".to_string(),
                    )
                })?
                .ok_or_else(|| {
                    AppError::new(
                        AppErrorKind::ContractCallError,
                        "No transaction receipt".to_string(),
                    )
                })?;
            let tx_hash: String = tx.transaction_hash.encode_hex();
            println!("Tx Hash: '{tx_hash}'");
            Ok(0)
        }
        Err(err) => {
            let err = AppError::new(AppErrorKind::ContractCallError, err.to_string());
            Err(err)
        }
    }
}
