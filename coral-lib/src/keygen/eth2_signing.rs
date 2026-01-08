use anyhow::Result;
use blsttc::SecretKey;
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
    sk: &SecretKey,
    withdrawal_credentials: [u8; 32],
    fork_version: Version,
) -> Result<(BLSSignature, Root)> {
    let pk = sk.public_key();
    let deposit_message = DepositMessage {
        pubkey: pk.to_bytes().to_vec().into(),
        withdrawal_credentials,
        amount: FULL_DEPOSIT_AMOUNT,
    };

    let domain = compute_domain(DOMAIN_DEPOSIT, Some(fork_version), None);
    let root: Root = compute_signing_root(deposit_message.clone(), domain);
    let sig: BLSSignature = BLSSignature::from(sk.sign(&root).to_bytes().to_vec());

    let dd = DepositData {
        pubkey: deposit_message.pubkey.clone(),
        withdrawal_credentials: deposit_message.withdrawal_credentials,
        amount: deposit_message.amount,
        signature: sig.clone(),
    };

    let dd_root = dd.tree_hash_root().to_fixed_bytes();

    Ok((sig, dd_root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_domain_with_genesis_fork_version() {
        let domain = compute_domain(DOMAIN_DEPOSIT, None, None);

        // Domain should be 32 bytes
        assert_eq!(domain.len(), 32);

        // First 4 bytes should be the domain type (DOMAIN_DEPOSIT = 0x03000000)
        assert_eq!(&domain[0..4], &DOMAIN_DEPOSIT);
    }

    #[test]
    fn test_compute_domain_with_custom_fork_version() {
        let fork_version: Version = [0x01, 0x00, 0x00, 0x00]; // Mainnet
        let domain = compute_domain(DOMAIN_DEPOSIT, Some(fork_version), None);

        // Domain should be 32 bytes
        assert_eq!(domain.len(), 32);

        // First 4 bytes should be the domain type
        assert_eq!(&domain[0..4], &DOMAIN_DEPOSIT);
    }

    #[test]
    fn test_compute_domain_different_fork_versions_differ() {
        let fork_v1: Version = [0x01, 0x00, 0x00, 0x00];
        let fork_v2: Version = [0x02, 0x00, 0x00, 0x00];

        let domain1 = compute_domain(DOMAIN_DEPOSIT, Some(fork_v1), None);
        let domain2 = compute_domain(DOMAIN_DEPOSIT, Some(fork_v2), None);

        // Different fork versions should produce different domains
        assert_ne!(domain1, domain2);
    }

    #[test]
    fn test_compute_fork_data_root() {
        let fork_version: Version = [0x00, 0x00, 0x00, 0x00];
        let genesis_validators_root = [0u8; 32];

        let fork_data_root = compute_fork_data_root(fork_version, genesis_validators_root);

        // Fork data root should be 32 bytes
        assert_eq!(fork_data_root.len(), 32);
    }

    #[test]
    fn test_compute_signing_root() {
        let deposit_message = DepositMessage {
            pubkey: vec![0u8; 48].into(),
            withdrawal_credentials: [0u8; 32],
            amount: FULL_DEPOSIT_AMOUNT,
        };
        let domain = compute_domain(DOMAIN_DEPOSIT, None, None);

        let signing_root = compute_signing_root(deposit_message, domain);

        // Signing root should be 32 bytes
        assert_eq!(signing_root.len(), 32);
    }

    #[test]
    fn test_sign_full_deposit() {
        let sk = SecretKey::random();
        let withdrawal_credentials = [0x01u8; 32]; // ETH1 withdrawal credentials
        let fork_version: Version = [0x00, 0x00, 0x00, 0x00];

        let result = sign_full_deposit(&sk, withdrawal_credentials, fork_version);
        assert!(result.is_ok());

        let (signature, deposit_data_root) = result.unwrap();

        // Signature should be 96 bytes
        assert_eq!(signature.len(), 96);

        // Deposit data root should be 32 bytes
        assert_eq!(deposit_data_root.len(), 32);
    }

    #[test]
    fn test_sign_full_deposit_signature_verification() {
        let sk = SecretKey::random();
        let pk = sk.public_key();
        let withdrawal_credentials = [0x01u8; 32];
        let fork_version: Version = [0x00, 0x00, 0x00, 0x00];

        let (signature, _) = sign_full_deposit(&sk, withdrawal_credentials, fork_version).unwrap();

        // Reconstruct the signing root to verify the signature
        let deposit_message = DepositMessage {
            pubkey: pk.to_bytes().to_vec().into(),
            withdrawal_credentials,
            amount: FULL_DEPOSIT_AMOUNT,
        };
        let domain = compute_domain(DOMAIN_DEPOSIT, Some(fork_version), None);
        let signing_root = compute_signing_root(deposit_message, domain);

        // Convert signature bytes back to blsttc Signature and verify
        let sig_bytes: [u8; 96] = signature[..].try_into().unwrap();
        let bls_sig = blsttc::Signature::from_bytes(sig_bytes).unwrap();
        assert!(pk.verify(&bls_sig, &signing_root));
    }

    #[test]
    fn test_sign_full_deposit_deterministic() {
        let sk = SecretKey::random();
        let withdrawal_credentials = [0x01u8; 32];
        let fork_version: Version = [0x00, 0x00, 0x00, 0x00];

        let (sig1, root1) = sign_full_deposit(&sk, withdrawal_credentials, fork_version).unwrap();
        let (sig2, root2) = sign_full_deposit(&sk, withdrawal_credentials, fork_version).unwrap();

        // Same inputs should produce same outputs
        assert_eq!(sig1[..], sig2[..]);
        assert_eq!(root1, root2);
    }

    #[test]
    fn test_sign_full_deposit_different_withdrawal_credentials() {
        let sk = SecretKey::random();
        let wc1 = [0x01u8; 32];
        let wc2 = [0x02u8; 32];
        let fork_version: Version = [0x00, 0x00, 0x00, 0x00];

        let (sig1, root1) = sign_full_deposit(&sk, wc1, fork_version).unwrap();
        let (sig2, root2) = sign_full_deposit(&sk, wc2, fork_version).unwrap();

        // Different withdrawal credentials should produce different signatures and roots
        assert_ne!(sig1[..], sig2[..]);
        assert_ne!(root1, root2);
    }

    #[test]
    fn test_deposit_amount_is_32_eth() {
        // Verify the constant is set correctly (32 ETH in Gwei)
        assert_eq!(FULL_DEPOSIT_AMOUNT, 32_000_000_000);
    }
}
