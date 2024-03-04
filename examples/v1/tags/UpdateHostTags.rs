// Update host tags returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_tags::TagsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body = HostTags::new().host("test.host".to_string()).tags(vec!["environment:production".to_string()]);
    let configuration = Configuration::new();
    let api = TagsAPI::with_config(configuration);
    let resp = api.update_host_tags(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
