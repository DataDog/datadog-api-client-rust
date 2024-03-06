// Update indexes order returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs_indexes::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body = LogsIndexesOrder::new(vec![
        "main".to_string(),
        "payments".to_string(),
        "web".to_string(),
    ]);
    let configuration = Configuration::new();
    let api = LogsIndexesAPI::with_config(configuration);
    let resp = api.update_logs_index_order(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
