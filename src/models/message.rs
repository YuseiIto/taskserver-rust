pub trait Message {
    fn size(&self) -> usize;
    fn msg_type() -> MessageType;
    fn protocol(&self) -> String;
    fn client(&self) -> ClientIdentifier;
}

pub enum MessageType {
    Sync,
    Statistics,
    Response,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Statistics
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientIdentifier {
    pub name: String,
    pub version: String,
}

impl Default for ClientIdentifier {
    fn default() -> Self {
        ClientIdentifier {
            name: "taskserver-rust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl From<String> for ClientIdentifier {
    fn from(s: String) -> Self {
        let mut parts = s.splitn(2, " ");
        Self {
            name: parts.next().unwrap().to_string(),
            version: parts.next().unwrap().to_string(),
        }
    }
}

pub struct AuthData {
    pub org: String,
    pub user: String,
    pub key: String,
}

pub struct Header {
    name: String,
    value: String,
}

impl From<&str> for Header {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(2, ": ");
        let name = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        Header { name, value }
    }
}

impl Header {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn value(&self) -> String {
        self.value.to_string()
    }
}
