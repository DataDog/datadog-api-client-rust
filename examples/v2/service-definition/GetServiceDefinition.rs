// Get a single service definition returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_definition::ServiceDefinitionAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ServiceDefinitionAPI::with_config(configuration);
    let resp = api.get_service_definition().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
