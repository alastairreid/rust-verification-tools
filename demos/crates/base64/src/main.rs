/// Test base64 crate
///
/// cargo verify --tests -v --script=log --replay --backend-flags=--search=dfs
/// for i in kleeout/test_decode_config/test*.ktest; do KTEST_FILE=$i cargo test --features verifier-klee test_decode_config -- --nocapture; done

use verification_annotations as verifier;
use base64::*;

const N: usize = 10;

fn abstract_charset() -> CharacterSet {
    match verifier::AbstractValue::abstract_where(|x| *x < 6u32) {
        0 => CharacterSet::Standard,
        1 => CharacterSet::UrlSafe,
        2 => CharacterSet::Crypt,
        3 => CharacterSet::Bcrypt,
        4 => CharacterSet::ImapMutf7,
        5 => CharacterSet::BinHex,
        _ => verifier::verifier_unreachable!()
    }
}

fn abstract_Config() -> Config {
    let char_set = abstract_charset();
    let pad = verifier::AbstractValue::abstract_value();
    let trailing = verifier::AbstractValue::abstract_value();
    let cfg = Config::new(char_set, pad);
    cfg.decode_allow_trailing_bits(trailing)
}

#[test]
fn test_decode_config() {
    let data = verifier::verifier_nondet_bytes(N);
    let cfg = abstract_Config();

    let result = decode_config(data, cfg);
    if verifier::is_replay() {
        println!("{:?} ({:?})", result, cfg);
    }
}

#[test]
fn test_roundtrip() {
    let data = verifier::verifier_nondet_bytes(N);
    let cfg = abstract_Config();

    let encoded = base64::encode_config(&data, cfg);
    let decoded = base64::decode_config(&encoded, cfg).unwrap();
    assert_eq!(data, decoded.as_slice());
}
