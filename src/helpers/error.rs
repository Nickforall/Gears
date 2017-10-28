use base64;

#[derive(Debug)]
pub struct ErrorBag {
    messages: Vec<String>,
}

impl ErrorBag {
    pub fn new() -> Self {
        ErrorBag {
            messages: Vec::new()
        }
    }

    pub fn add(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }

    pub fn has_errors(&self) -> bool {
        !self.messages.is_empty()
    }

    pub fn encode(&self) -> String {
        // I explicitely choose not to make a comma seperated value, because my errors will contain
        // comma's
        return base64::encode(self.messages.join("#").as_str())
    }

    pub fn as_vec(&self) -> Vec<String> {
        return self.messages.clone();
    }

    pub fn decode(b64: &String) -> Self {
        let mut bag = ErrorBag {
            messages: Vec::new()
        };

        let rust_str = String::from_utf8(base64::decode(b64.as_str()).unwrap()).unwrap();

        for s in rust_str.split("#") {
            bag.add(s);
        }

        bag
    }
}
