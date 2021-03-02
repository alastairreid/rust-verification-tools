use verification_annotations as verifier;

fn main() {
    // verifier::verifier_warning("Warn1");
    // verifier::verifier_warning("Warn2");
    // verifier::verifier_warning("Warn1");
    // verifier::verifier_warning_once("Warn1");
    // verifier::verifier_warning_once("Warn2");
    // verifier::verifier_warning_once("Warn1");

    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(1 <= a && a <= 1000);
    verifier::assume(1 <= b && b <= 1000);

    if verifier::is_replay() {
        eprintln!("  Test values: a = {}, b = {}", a, b);
    }

    verifier::case_split(a < 500);
    verifier::case_split(b < 200);

    // verifier::coherent! { verifier::case_split(b < 200); }
    // verifier::coherent! {
    //     let c = verifier::any_of(&[1, 10, 100]);
    //     println!("Choice {}", c);
    // }

    let mut a = a;
    // This samples the legal values of a but does not check all
    // possible values so important corner cases can be missed
    if false {
        if false {
            verifier::coherent! {
                a = verifier::random_sample_u32(6, a);
            };
        } else {
            // a = verifier::sample_u32(16, a);
            a = verifier::random_sample_u32(5, a);
        }
        // println!("  sampled a = {}", a);
    }

    let r = a*b;
    verifier::assert!(1 <= r && r < 1000000);
}
