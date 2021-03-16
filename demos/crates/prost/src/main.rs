/// Test output generated by prost tool
///
/// # May have to interrupt this command because test_shirt_nofail takes a long time to run
/// cargo verify --tests -v
/// for i in kleeout/test_shirt_roundtrip/test*.ktest; do KTEST_FILE=$i cargo test --features verifier-klee test_shirt_roundtrip -- --nocapture; done
/// for i in kleeout/test_shirt_nofail/test*.ktest; do ktest-tool $i; done | grep data

use verification_annotations as verifier;
use prost::Message;


fn main() {
    println!("Hello, world!");
}

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}

fn abstract_size() -> items::shirt::Size {
    match verifier::AbstractValue::abstract_where(|x| *x < 3u32) {
        0 => items::shirt::Size::Small,
        1 => items::shirt::Size::Medium,
        2 => items::shirt::Size::Large,
        _ => verifier::verifier_unreachable!()
    }
}

fn abstract_shirt(color_len: usize) -> items::Shirt {
    let size = abstract_size();
    let color = verifier::verifier_nondet_ascii_string(color_len);
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(size);
    shirt
}

/// Test that encoding followed by decoding cannot fail
/// and cannot panic (or any other form of runtime exception)
#[test]
fn test_shirt_roundtrip() {
    const N: usize = 10;

    let shirt = abstract_shirt(N);

    let mut enc = Vec::new(); // buffer for encoded message
    enc.reserve(shirt.encoded_len());
    shirt.encode(&mut enc).unwrap(); // should not be able to fail?
    let enc = &enc[..];

    let dec = items::Shirt::decode(enc).unwrap(); // also cannot fail?

    if verifier::is_replay() {
        println!("{:?} --> {:?} --> {:?}", shirt, enc, dec);
        println!("len = {}", enc.len());
    }

    assert_eq!(dec, shirt)
}

/// Test that decoding process cannot panic
/// (This runs for a really long time - I have not seen it terminate.)
#[test]
fn test_shirt_nofail() {
    const N: usize = 8;
    let data = verifier::verifier_nondet_bytes(N);
    let data = &data[..];
    let _dec = items::Shirt::decode(data);
}
