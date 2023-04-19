pub struct Message(pub String);

pub struct Printer;

impl Message {
    pub fn get_message(&self) -> String {
        self.0.to_string()
    }
}

impl Printer {
    pub fn println(&self, msg: impl Into<String>) {
        println!("{}", msg.into())
    }
}