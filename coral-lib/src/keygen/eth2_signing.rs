use anyhow::Result;
use blsttc::SecretKeySet;
use ssz::Encode;
use tree_hash::TreeHash;

use super::constants::FULL_DEPOSIT_AMOUNT;
use super::eth2_types::*;

/// Return the signing root for the corresponding signing data.
pub fn compute_signing_root<T: Encode + TreeHash>(ssz_object: T, domain: Domain) -> Root {
    let object_root = ssz_object.tree_hash_root().to_fixed_bytes();
    let sign_data = SigningData {
        object_root,
        domain,
    };
    sign_data.tree_hash_root().to_fixed_bytes()
}

/// Return the 32-byte fork data root for the `current_version` and `genesis_validators_root`.
/// This is used primarily in signature domains to avoid collisions across forks/chains.
pub fn compute_fork_data_root(current_version: Version, genesis_validators_root: Root) -> Root {
    let f = ForkData {
        current_version,
        genesis_validators_root,
    };
    f.tree_hash_root().to_fixed_bytes()
}

/// Return the domain for the `domain_type` and `fork_version`.
pub fn compute_domain(
    domain_type: DomainType,
    fork_version: Option<Version>,
    genesis_validators_root: Option<Root>,
) -> Domain {
    let fv = fork_version.unwrap_or(GENESIS_FORK_VERSION);
    let gvr = genesis_validators_root.unwrap_or(Root::default());
    let fork_data_root = compute_fork_data_root(fv, gvr);
    let mut d = [0_u8; 32]; // domain_type + fork_data_root[:28]
    domain_type.iter().enumerate().for_each(|(i, v)| d[i] = *v);
    d[4..32]
        .iter_mut()
        .zip(fork_data_root[0..28].iter())
        .for_each(|(src, dest)| *src = *dest);
    d
}

/// Sign a full deposit message and return the signature and deposit data root
pub fn sign_full_deposit(
    sk_set: &SecretKeySet,
    withdrawal_credentials: [u8; 32],
    fork_version: Version,
) -> Result<(BLSSignature, Root)> {
    let deposit_message = DepositMessage {
        pubkey: sk_set.public_keys().public_key().to_bytes().to_vec().into(),
        withdrawal_credentials,
        amount: FULL_DEPOSIT_AMOUNT,
    };

    let domain = compute_domain(DOMAIN_DEPOSIT, Some(fork_version), None);
    let root: Root = compute_signing_root(deposit_message.clone(), domain);
    let sig: BLSSignature = BLSSignature::from(sk_set.secret_key().sign(&root).to_bytes().to_vec());

    let dd = DepositData {
        pubkey: deposit_message.pubkey.clone(),
        withdrawal_credentials: deposit_message.withdrawal_credentials,
        amount: deposit_message.amount,
        signature: sig.clone(),
    };

    let dd_root = dd.tree_hash_root().to_fixed_bytes();

    Ok((sig, dd_root))
}

