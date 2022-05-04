pub trait Message {
    fn msg_type() -> MessageType;
    fn protocol(&self) -> String;
    fn client(&self) -> ClientIdentifier;

    fn parse_size(text: &str) -> usize {
        let size: String = text.chars().take_while(|c| c.is_digit(10)).collect();
        return size.parse::<usize>().unwrap();
    }

    fn parse_headers(text: &str) -> Vec<Header> {
        let size_digit_number = Self::parse_size(text).to_string().len();
        let header = text.chars().skip(size_digit_number).collect::<String>();
        let lines = header
            .lines()
            .take_while(|&line| line.len() > 0)
            .collect::<Vec<&str>>();

        lines
            .iter()
            .map(|&x| Header::from(x))
            .collect::<Vec<Header>>()
    }

    fn collect_headers(&self) -> Vec<Header>;
    fn collect_body(&self) -> Option<String>;
    fn size(&self) -> usize {
        let dump = self.dump();
        dump.len()
    }

    fn dump(&self) -> String {
        let headers = self.collect_headers();
        let body = self.collect_body();

        let mut result = String::new();

        for header in headers.iter() {
            result.push_str(&format!("{}: {}\n", header.name, header.value));
        }

        result.push_str("\n");

        if let Some(body) = body {
            result.push_str(&body);
        }

        result
    }

    fn dump_with_size(&self) -> String {
        let dump = self.dump();
        let size = dump.len().to_string();
        format!("{}{}", size, dump)
    }
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

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}
impl From<&str> for Header {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(2, ": ");
        let name = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        Header { name, value }
    }
}
