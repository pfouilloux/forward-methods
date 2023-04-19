use crate::stubs::{Message, Printer};

mod stubs;

#[derive(forward_methods::Composite)]
struct CompositeStruct(Message, Printer);

#[test]
fn should_forward_public_methods_from_all_fields_by_default() {
    let cmp = CompositeStruct(Message("hello, world!".to_string()), Printer);

    cmp.println(cmp.get_message())
}