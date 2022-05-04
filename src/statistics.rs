use crate::models::message::Message;

use super::models;
use models::request::statistics::StatisticsRequest;

fn get_statisics() {
    let org = "public".to_string();
    let user = "yuseiito".to_string();
    let key = "be0115fa-9b1f-4e85-8238-4cc147b91f7f".to_string();

    let req = StatisticsRequest {
        org,
        user,
        key,
        ..StatisticsRequest::default()
    };

    let msg = req.dump_with_size();
    println!("{}", msg);
}

#[test]

fn test_get_statistics() {
    get_statisics();
}
