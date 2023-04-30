pub struct Message(pub String);

pub struct Printer;

impl Message {
    pub fn get_message(&self) -> String {
        self.0.to_string()
    }
    pub fn get_len(&self) -> usize {
        self.0.len()
    }
}

impl Printer {
    pub fn println(&self, msg: impl Into<String>) {
        println!("{}", msg.into())
    }
}
