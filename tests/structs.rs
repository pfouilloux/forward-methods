use crate::stubs::{Message, Printer};

mod stubs;

#[forward_methods::composite]
struct CompositeStruct {
    msg: Message,
    printer: Printer,
}

#[test]
fn should_forward_public_methods_from_all_fields_by_default() {
    let cmp = CompositeStruct {
        msg: Message("hello, world!".to_string()),
        printer: Printer,
    };

    cmp.println(cmp.get_message());
}