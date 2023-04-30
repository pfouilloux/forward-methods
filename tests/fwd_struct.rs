use forward_methods::fwd;

use crate::stubs::{Message, Printer};

mod stubs;

struct CompositeStruct {
    message: Message,
    printer: Printer,
}

impl CompositeStruct {
    fwd!(fn get_message(&self) -> String, fn get_len(&self) -> usize to self.message);
    fwd!(fn println(&self, msg: impl Into<String>) to self.printer);
}

#[test]
fn should_forward_methods() {
    let cmp = CompositeStruct {
        message: Message("hello, world!".to_string()),
        printer: Printer,
    };

    cmp.println(format!("{}: {}", cmp.get_message(), cmp.get_len()))
}
