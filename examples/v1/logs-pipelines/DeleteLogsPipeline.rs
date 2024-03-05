// Delete a pipeline returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsPipelinesAPI::with_config(configuration);
    let resp = api.delete_logs_pipeline().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
