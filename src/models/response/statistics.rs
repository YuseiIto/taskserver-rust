use super::super::message::{ClientIdentifier, Header, Message, MessageType};
use super::{Response, StatusCode};
use log::info;

#[derive(Debug, Default)]
pub struct StatisticsResponse {
    size: usize,
    client: ClientIdentifier,
    protocol: String,
    code: StatusCode,
    status: String,
    average_request_bytes: usize,
    average_response_bytes: usize,
    average_response_time: f32,
    errors: u32,
    idle: f32,
    maximum_response_time: f32,
    total_bytes_in: usize,
    total_bytes_out: usize,
    tps: f32,
    transactions: u32,
    uptime: u32,
}

impl Response for StatisticsResponse {
    fn code(&self) -> StatusCode {
        self.code.clone()
    }

    fn status(&self) -> String {
        self.status.clone()
    }
}

impl Message for StatisticsResponse {
    fn msg_type() -> MessageType {
        MessageType::Response
    }

    fn size(&self) -> usize {
        self.size
    }

    fn client(&self) -> ClientIdentifier {
        self.client.clone()
    }

    fn protocol(&self) -> String {
        self.protocol.to_string()
    }
}

impl StatisticsResponse {
    pub fn average_request_bytes(&self) -> usize {
        self.average_request_bytes
    }

    pub fn average_response_bytes(&self) -> usize {
        self.average_response_bytes
    }

    pub fn average_response_time(&self) -> f32 {
        self.average_response_time
    }

    pub fn errors(&self) -> u32 {
        self.errors
    }

    pub fn idle(&self) -> f32 {
        self.idle
    }

    pub fn maximum_response_time(&self) -> f32 {
        self.maximum_response_time
    }

    pub fn total_bytes_in(&self) -> usize {
        self.total_bytes_in
    }
    pub fn total_bytes_ou(&self) -> usize {
        self.total_bytes_out
    }
    pub fn tps(&self) -> f32 {
        self.tps
    }
    pub fn transactions(&self) -> u32 {
        self.transactions
    }
    pub fn uptime(&self) -> u32 {
        self.uptime
    }

    pub fn parse(text: &str) -> Self {
        let size: String = text.chars().take_while(|c| c.is_digit(10)).collect();
        let size_digit_number = size.len();
        let size = size.parse::<usize>().unwrap();

        let header = text.chars().skip(size_digit_number).collect::<String>();
        let lines = header
            .lines()
            .take_while(|&line| line.len() > 0)
            .collect::<Vec<&str>>();

        let mut res = StatisticsResponse {
            size,
            ..StatisticsResponse::default()
        };

        for line in lines {
            let header = Header::from(line);

            match header.name().as_str() {
                "client" => res.client = ClientIdentifier::from(header.value()),
                "protocol" => res.protocol = header.value(),
                "code" => res.code = StatusCode::from(header.value().parse::<u16>().unwrap()),
                "status" => res.status = header.value(),
                "average request bytes" => {
                    res.average_request_bytes = header.value().parse::<usize>().unwrap()
                }
                "average response bytes" => {
                    res.average_response_bytes = header.value().parse::<usize>().unwrap()
                }
                "average response time" => {
                    res.average_response_time = header.value().parse::<f32>().unwrap()
                }
                "errors" => res.errors = header.value().parse::<u32>().unwrap(),
                "idle" => res.idle = header.value().parse::<f32>().unwrap(),
                "maximum response time" => {
                    res.maximum_response_time = header.value().parse::<f32>().unwrap()
                }
                "total bytes in" => res.total_bytes_in = header.value().parse::<usize>().unwrap(),
                "total bytes out" => res.total_bytes_out = header.value().parse::<usize>().unwrap(),
                "tps" => res.tps = header.value().parse::<f32>().unwrap(),
                "transactions" => res.transactions = header.value().parse::<u32>().unwrap(),
                "uptime" => res.uptime = header.value().parse::<u32>().unwrap(),
                _ => info!("Encounterd unknown header: {}", header.name()),
            }
        }

        res
    }
}

#[test]
fn parse_statstics_response() {
    let input = r#"287type: response
client: taskd 1.0.0
protocol: v1
code: 200
status: Ok
average request bytes: 0
average response bytes: 0
average response time: 0.000000
errors: 0
idle: 1.000000
maximum response time: 0.000000
total bytes in: 0
total bytes out: 0
tps: 0.000000
transactions: 1
uptime: 28
"#;

    let res = StatisticsResponse::parse(input);

    assert_eq!(
        res.client,
        ClientIdentifier {
            name: "taskd".to_string(),
            version: "1.0.0".to_string()
        }
    );
    assert_eq!(res.protocol, "v1");
    assert_eq!(res.code, StatusCode::Success);
    assert_eq!(res.average_request_bytes, 0);
    assert_eq!(res.average_response_bytes, 0);
    assert_eq!(res.average_response_time, 0.0);
    assert_eq!(res.errors, 0);
    assert_eq!(res.idle, 1.0);
    assert_eq!(res.maximum_response_time, 0.0);
    assert_eq!(res.total_bytes_in, 0);
    assert_eq!(res.total_bytes_out, 0);
    assert_eq!(res.tps, 0.0);
    assert_eq!(res.transactions, 1);
    assert_eq!(res.uptime, 28);
}
