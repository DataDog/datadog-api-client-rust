// Get indexes order returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs_indexes::LogsIndexesAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsIndexesAPI::with_config(configuration);
    let resp = api.get_logs_index_order().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
