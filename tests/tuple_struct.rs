use forward_methods::fwd;

use crate::stubs::{Message, Printer};

mod stubs;

struct CompositeStruct(Message, Printer);

impl CompositeStruct {
    fwd!(fn get_message(&self) -> String, fn get_len(&self) -> usize to self.0);
    fwd!(fn println(&self, msg: impl Into<String>) to self.1);
}

#[test]
fn should_forward_methods() {
    let cmp = CompositeStruct(Message("hello, world!".to_string()), Printer);

    cmp.println(format!("{}: {}", cmp.get_message(), cmp.get_len()))
}
