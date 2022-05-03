use super::message::Header;
pub mod statistics;

trait Request {
    fn collect_headers(&self) -> Vec<Header>;
    fn collect_body(&self) -> Option<String>;
}
