mod private {
    use forward_methods::fwd_pub;

    pub struct CompositeStruct {
        pub message: String,
    }

    impl CompositeStruct {
        fwd_pub!(fn len(&self) -> usize to self.message);
    }
}

#[test]
fn should_forward_methods() {
    let cmp = private::CompositeStruct {
        message: "hello, world!".to_string(),
    };

    println!("string len: {}", cmp.len())
}
