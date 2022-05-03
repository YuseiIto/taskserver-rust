use super::super::message::{Auth, ClientIdentifier, Header, Message, MessageType};
use super::Request;

struct StatisticsRequest {
    org: String,
    user: String,
    key: String,
    client: ClientIdentifier,
    protocol: String,
}

impl Message for StatisticsRequest {
    fn size(&self) -> usize {
        0
    }

    fn msg_type() -> MessageType {
        MessageType::Statistics
    }

    fn protocol(&self) -> String {
        self.protocol.clone()
    }
    fn client(&self) -> ClientIdentifier {
        self.client.clone()
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

impl Request for StatisticsRequest {
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
