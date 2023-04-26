use crate::stubs::{Message, Printer};
use forward_methods::fwd;

mod stubs;

struct CompositeStruct(Message, Printer);

impl CompositeStruct {
    fwd!(get_message(&self) -> String => self.0);
    fwd!(println(&self, msg: impl Into<String>) => self.1);
}

#[test]
fn should_forward_public_methods_from_all_fields_by_default() {
    let cmp = CompositeStruct(Message("hello, world!".to_string()), Printer);

    cmp.println(cmp.get_message())
}
