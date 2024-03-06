// Get all Container Images returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_container_images::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ContainerImagesAPI::with_config(configuration);
    let resp = api
        .list_container_images(ListContainerImagesOptionalParams::default().page_size(2))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
