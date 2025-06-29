use rust_core::keygen::ephemeral_key;

#[test]
fn keys_are_unique() {
    let k1 = ephemeral_key();
    let k2 = ephemeral_key();
    assert_ne!(k1, k2, "Generated keys should be unique");
}
