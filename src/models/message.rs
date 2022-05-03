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

impl Into<String> for MessageType {
    fn into(self) -> String {
        match self {
            MessageType::Sync => "sync".to_string(),
            MessageType::Statistics => "statistics".to_string(),
            MessageType::Response => "response".to_string(),
        }
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

impl Into<String> for ClientIdentifier {
    fn into(self) -> String {
        format!("{} {}", self.name, self.version)
    }
}

pub trait Auth {
    fn org(&self) -> String;
    fn user(&self) -> String;
    fn key(&self) -> String;
}

pub struct Header {
    pub name: String,
    pub value: String,
}

impl From<&str> for Header {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(2, ": ");
        let name = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        Header { name, value }
    }
}
