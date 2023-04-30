mod private {
    use forward_methods::fwd_pub;

    pub struct CompositeStruct(pub String);

    impl CompositeStruct {
        fwd_pub!(fn len(&self) -> usize to self.0);
    }
}

#[test]
fn should_forward_methods() {
    let cmp = private::CompositeStruct("hello, world!".to_string());

    println!("string len: {}", cmp.len())
}
