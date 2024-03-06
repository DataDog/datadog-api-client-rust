// Get all service definitions returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_definition::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ServiceDefinitionAPI::with_config(configuration);
    let resp = api
        .list_service_definitions(ListServiceDefinitionsOptionalParams::default().page_size(2))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}