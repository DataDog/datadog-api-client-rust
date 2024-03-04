// Send logs returns "Response from server (always 200 empty JSON)." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs::LogsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        vec![
            HTTPLogItem::new("Example-Log".to_string())
                .ddtags("host:ExampleLog".to_string())
                .additional_properties(std::collections::BTreeMap::from([]))
        ];
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api.submit_log(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
