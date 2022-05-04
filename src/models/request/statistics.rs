use super::super::message::{Auth, ClientIdentifier, Header, Message, MessageType};
use super::Request;

#[derive(Debug)]
pub struct StatisticsRequest {
    pub org: String,
    pub user: String,
    pub key: String,
    pub client: ClientIdentifier,
    pub protocol: String,
}

impl Request for StatisticsRequest {}

impl Default for StatisticsRequest {
    fn default() -> Self {
        StatisticsRequest {
            org: String::default(),
            user: String::default(),
            key: String::default(),
            client: ClientIdentifier::default(),
            protocol: "v1".to_string(),
        }
    }
}

impl Message for StatisticsRequest {
    fn msg_type() -> MessageType {
        MessageType::Statistics
    }

    fn protocol(&self) -> String {
        self.protocol.clone()
    }
    fn client(&self) -> ClientIdentifier {
        self.client.clone()
    }

    fn collect_headers(&self) -> Vec<Header> {
        let mut headers = Vec::new();

        headers.push(Header {
            name: "type".to_string(),
            value: MessageType::Statistics.into(),
        });

        headers.push(Header {
            name: "org".to_string(),
            value: self.org(),
        });

        headers.push(Header {
            name: "user".to_string(),
            value: self.user.clone(),
        });

        headers.push(Header {
            name: "key".to_string(),
            value: self.key.clone(),
        });

        headers.push(Header {
            name: "client".to_string(),
            value: self.client().into(),
        });

        headers.push(Header {
            name: "protocol".to_string(),
            value: self.protocol.clone(),
        });

        headers
    }

    fn collect_body(&self) -> Option<String> {
        None
    }
}

impl Auth for StatisticsRequest {
    fn org(&self) -> String {
        self.org.clone()
    }
    fn user(&self) -> String {
        self.user.clone()
    }
    fn key(&self) -> String {
        self.key.clone()
    }
}
