// Send gzip logs returns "Request accepted for processing (always 202 empty
// JSON)." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = vec![HTTPLogItem::new(
        "2019-11-19T14:37:58,995 INFO [process.name][20081] Hello World".to_string(),
    )
    .ddsource("nginx".to_string())
    .ddtags("env:staging,version:5.1".to_string())
    .hostname("i-012345678".to_string())
    .service("payment".to_string())
    .additional_properties(BTreeMap::from([]))];
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api
        .submit_log(
            body,
            SubmitLogOptionalParams::default().content_encoding(ContentEncoding::GZIP),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}