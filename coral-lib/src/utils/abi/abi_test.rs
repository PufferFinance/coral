// use ethers::utils::hex::{FromHex, ToHex};
// use puffersecuresigner::{
//     enclave::types::KeyGenResponse, io::remote_attestation::AttestationEvidence,
// };
// use crate::utils;

/*
const KEYGEN_TEST_RESPONSE: &str =
    include_str!("../../../../mock-server/test_data/keygen_response_001.json");

const REPORT_CALLDATA: &str =
    include_str!("../../../../mock-server/test_data/keygen_response_001_report_calldata.txt");
const RAVE_CALLDATA: &str =
    include_str!("../../../../mock-server/test_data/keygen_response_001_rave_calldata.txt");

#[test]
fn test_extract_x509_valid() {
    let report: KeyGenResponse = serde_json::from_str(KEYGEN_TEST_RESPONSE).unwrap();

    let (leaf_x509, root_x509) =
        utils::ssl::extract_x509(report.evidence.signing_cert.as_bytes()).unwrap();

    assert_eq!(
        true,
        utils::ssl::verify_intel_sgx_root_ca(&root_x509).is_ok()
    );
    assert_eq!(
        true,
        utils::ssl::verify_intel_sgx_attestation_report(&leaf_x509).is_ok()
    );
}

#[test]
fn test_raw_report_calldata_valid() {
    let report: KeyGenResponse = serde_json::from_str(KEYGEN_TEST_RESPONSE).unwrap();
    let evidence = &report.evidence;
    let raw_report =
        utils::abi::intel_report::deserialize_report(evidence.raw_report.as_bytes()).unwrap();

    let actual = utils::abi::intel_report::to_calldata(&raw_report).unwrap();
    let expected: Vec<u8> = <Vec<u8>>::from_hex(REPORT_CALLDATA).unwrap();

    assert_eq!(expected.len(), actual.len());
    assert_eq!(expected, actual);
}

#[test]
fn test_rave_calldata_generate_json() {
    const KEYGEN_RESPONSE_JSON: &str = include_str!("../../../../mock-server/test_data/0x8144e74faf431dad80831ef0dddca2302f9a913bc0c2fdfafaeacac05cdb173b.json");

    let report: KeyGenResponse = serde_json::from_str(KEYGEN_RESPONSE_JSON).unwrap();
    let attestation_report =
        utils::abi::intel_report::deserialize_report(report.evidence.raw_report.as_bytes())
            .unwrap();

    let report_json = utils::abi::intel_report::to_json(&attestation_report).unwrap();
    println!("{}", report_json);

    let rave_json = utils::abi::rave_evidence::to_json(&report, &attestation_report).unwrap();
    println!("{}", rave_json);

    let report_calldata = utils::abi::intel_report::to_calldata(&attestation_report).unwrap();
    println!(
        "report calldata: '{}'",
        report_calldata.encode_hex::<String>()
    );
}

#[test]
fn test_rave_calldata_generate_json_from_keygen() {
    let report = KeyGenResponse {
        pk_hex: "0x04a55b152177219971a93a64aafc2d61baeaf86526963caa260e71efa2b865527e0307d7bda85312dd6ff23bcc88f2bf228da6295239f72c31b686c48b7b69cdfd".to_string(),
        evidence: AttestationEvidence {
            raw_report: "{\"id\":\"186453210057823126547835745110511429060\",\"timestamp\":\"2023-10-16T20:17:59.516859\",\"version\":4,\"epidPseudonym\":\"EbrM6X6YCH3brjPXT23gVh/I2EG5sVfHYh+S54fb0rrAqVRTiRTOSfLsWSVTZc8wrazGG7oooGoMU7Gj5TEhspNWPNBkpcmwf+3WZYsuncw6eX6Uijk+PzPp3dBQSebHsOEQYDRxGeFuWowvkTo2Z5HTavyoRIrSupBTqDE78HA=\",\"advisoryURL\":\"https://security-center.intel.com\",\"advisoryIDs\":[\"INTEL-SA-00334\",\"INTEL-SA-00615\"],\"isvEnclaveQuoteStatus\":\"SW_HARDENING_NEEDED\",\"isvEnclaveQuoteBody\":\"AgABAKwMAAANAA0AAAAAAEJhbJjVPJcSY5RHybDnAD8AAAAAAAAAAAAAAAAAAAAAFRULB/+ADgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABQAAAAAAAAAfAAAAAAAAAI63DnajS/bL+d7tf0Z7SIjMGH8fHzTM4tEcpUAUFJo1AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACGGfbNg35+DFwYIjUiKm+84K8beqRS7hF/CO4eKrj1YAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"}".to_string(),
            signed_report: "LDx4Yimeh/lB5amkm5ahvJ4v7E/BXp6mTow9akiUy+9DMkisIUAvFlBik2sS/9dea4xAFcNJkbdaVYOQQ03LcKGE4k/sOPUJKYJiga4xeT6gL8O1HwCZBun+u/IKvs7jKy27gx9ne6usvEBXAwxGUK1Ng9xbayiu94tJwmd3wEgghWvTbFFPDoaTwLp82UoZHSQY8rDX0lMwE+WpgVs8t4WwV1ZIJnt5kMBwsmYUsd4Ylq3mORNkFX6+BhEreohP8FlYbLpPaKAqJEATbhb1ubZb4NIII863c8khkfxhXLq1aLgJtb7q1HIBFhI4uWgV8OnSw3xEAOSseU3PKxs1mQ==".to_string(),
            signing_cert: "-----BEGIN CERTIFICATE-----\nMIIEoTCCAwmgAwIBAgIJANEHdl0yo7CWMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNV\nBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNV\nBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0\nYXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwHhcNMTYxMTIyMDkzNjU4WhcNMjYxMTIw\nMDkzNjU4WjB7MQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExFDASBgNVBAcMC1Nh\nbnRhIENsYXJhMRowGAYDVQQKDBFJbnRlbCBDb3Jwb3JhdGlvbjEtMCsGA1UEAwwk\nSW50ZWwgU0dYIEF0dGVzdGF0aW9uIFJlcG9ydCBTaWduaW5nMIIBIjANBgkqhkiG\n9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqXot4OZuphR8nudFrAFiaGxxkgma/Es/BA+t\nbeCTUR106AL1ENcWA4FX3K+E9BBL0/7X5rj5nIgX/R/1ubhkKWw9gfqPG3KeAtId\ncv/uTO1yXv50vqaPvE1CRChvzdS/ZEBqQ5oVvLTPZ3VEicQjlytKgN9cLnxbwtuv\nLUK7eyRPfJW/ksddOzP8VBBniolYnRCD2jrMRZ8nBM2ZWYwnXnwYeOAHV+W9tOhA\nImwRwKF/95yAsVwd21ryHMJBcGH70qLagZ7Ttyt++qO/6+KAXJuKwZqjRlEtSEz8\ngZQeFfVYgcwSfo96oSMAzVr7V0L6HSDLRnpb6xxmbPdqNol4tQIDAQABo4GkMIGh\nMB8GA1UdIwQYMBaAFHhDe3amfrzQr35CN+s1fDuHAVE8MA4GA1UdDwEB/wQEAwIG\nwDAMBgNVHRMBAf8EAjAAMGAGA1UdHwRZMFcwVaBToFGGT2h0dHA6Ly90cnVzdGVk\nc2VydmljZXMuaW50ZWwuY29tL2NvbnRlbnQvQ1JML1NHWC9BdHRlc3RhdGlvblJl\ncG9ydFNpZ25pbmdDQS5jcmwwDQYJKoZIhvcNAQELBQADggGBAGcIthtcK9IVRz4r\nRq+ZKE+7k50/OxUsmW8aavOzKb0iCx07YQ9rzi5nU73tME2yGRLzhSViFs/LpFa9\nlpQL6JL1aQwmDR74TxYGBAIi5f4I5TJoCCEqRHz91kpG6Uvyn2tLmnIdJbPE4vYv\nWLrtXXfFBSSPD4Afn7+3/XUggAlc7oCTizOfbbtOFlYA4g5KcYgS1J2ZAeMQqbUd\nZseZCcaZZZn65tdqee8UXZlDvx0+NdO0LR+5pFy+juM0wWbu59MvzcmTXbjsi7HY\n6zd53Yq5K244fwFHRQ8eOB0IWB+4PfM7FeAApZvlfqlKOlLcZL2uyVmzRkyR5yW7\n2uo9mehX44CiPJ2fse9Y6eQtcfEhMPkmHXI01sN+KwPbpA39+xOsStjhP9N1Y1a2\ntQAVo+yVgLgV2Hws73Fc0o3wC78qPEA+v2aRs/Be3ZFDgDyghc/1fgU+7C+P6kbq\nd4poyb6IW8KCJbxfMJvkordNOgOUUxndPHEi/tb/U7uLjLOgPA==\n-----END CERTIFICATE-----\n-----BEGIN CERTIFICATE-----\nMIIFSzCCA7OgAwIBAgIJANEHdl0yo7CUMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNV\nBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNV\nBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0\nYXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwIBcNMTYxMTE0MTUzNzMxWhgPMjA0OTEy\nMzEyMzU5NTlaMH4xCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwL\nU2FudGEgQ2xhcmExGjAYBgNVBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQD\nDCdJbnRlbCBTR1ggQXR0ZXN0YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwggGiMA0G\nCSqGSIb3DQEBAQUAA4IBjwAwggGKAoIBgQCfPGR+tXc8u1EtJzLA10Feu1Wg+p7e\nLmSRmeaCHbkQ1TF3Nwl3RmpqXkeGzNLd69QUnWovYyVSndEMyYc3sHecGgfinEeh\nrgBJSEdsSJ9FpaFdesjsxqzGRa20PYdnnfWcCTvFoulpbFR4VBuXnnVLVzkUvlXT\nL/TAnd8nIZk0zZkFJ7P5LtePvykkar7LcSQO85wtcQe0R1Raf/sQ6wYKaKmFgCGe\nNpEJUmg4ktal4qgIAxk+QHUxQE42sxViN5mqglB0QJdUot/o9a/V/mMeH8KvOAiQ\nbyinkNndn+Bgk5sSV5DFgF0DffVqmVMblt5p3jPtImzBIH0QQrXJq39AT8cRwP5H\nafuVeLHcDsRp6hol4P+ZFIhu8mmbI1u0hH3W/0C2BuYXB5PC+5izFFh/nP0lc2Lf\n6rELO9LZdnOhpL1ExFOq9H/B8tPQ84T3Sgb4nAifDabNt/zu6MmCGo5U8lwEFtGM\nRoOaX4AS+909x00lYnmtwsDVWv9vBiJCXRsCAwEAAaOByTCBxjBgBgNVHR8EWTBX\nMFWgU6BRhk9odHRwOi8vdHJ1c3RlZHNlcnZpY2VzLmludGVsLmNvbS9jb250ZW50\nL0NSTC9TR1gvQXR0ZXN0YXRpb25SZXBvcnRTaWduaW5nQ0EuY3JsMB0GA1UdDgQW\nBBR4Q3t2pn680K9+QjfrNXw7hwFRPDAfBgNVHSMEGDAWgBR4Q3t2pn680K9+Qjfr\nNXw7hwFRPDAOBgNVHQ8BAf8EBAMCAQYwEgYDVR0TAQH/BAgwBgEB/wIBADANBgkq\nhkiG9w0BAQsFAAOCAYEAeF8tYMXICvQqeXYQITkV2oLJsp6J4JAqJabHWxYJHGir\nIEqucRiJSSx+HjIJEUVaj8E0QjEud6Y5lNmXlcjqRXaCPOqK0eGRz6hi+ripMtPZ\nsFNaBwLQVV905SDjAzDzNIDnrcnXyB4gcDFCvwDFKKgLRjOB/WAqgscDUoGq5ZVi\nzLUzTqiQPmULAQaB9c6Oti6snEFJiCQ67JLyW/E83/frzCmO5Ru6WjU4tmsmy8Ra\nUd4APK0wZTGtfPXU7w+IBdG5Ez0kE1qzxGQaL4gINJ1zMyleDnbuS8UicjJijvqA\n152Sq049ESDz+1rRGc2NVEqh1KaGXmtXvqxXcTB+Ljy5Bw2ke0v8iGngFBPqCTVB\n3op5KBG3RjbF6RRSzwzuWfL7QErNC8WEy5yDVARzTA5+xmBc388v9Dm21HGfcC8O\nDD+gT9sSpssq0ascmvH49MOgjt1yoysLtdCtJW/9FZpoOypaHx0R+mJTLwPXVMrv\nDaVzWh5aiEx+idkSGMnX\n-----END CERTIFICATE-----\n".to_string(),
        },
    };
    let attestation_report =
        utils::abi::intel_report::deserialize_report(report.evidence.raw_report.as_bytes());
    let attestation_report = attestation_report.unwrap();

    println!("===========REPORT============");
    let report_json = utils::abi::intel_report::to_json(&attestation_report).unwrap();
    println!("{}", report_json);

    println!("===========RAVE============");
    let rave_json = utils::abi::rave_evidence::to_json(&report, &attestation_report).unwrap();
    println!("{}", rave_json);

    println!("==========CALLDATA=========");
    let report_calldata = utils::abi::intel_report::to_calldata(&attestation_report).unwrap();
    println!("{}", report_calldata.encode_hex::<String>());
}
 */
