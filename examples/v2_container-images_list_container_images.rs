// Get all Container Images returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api::api_container_images::ContainerImagesAPI;
use datadog_api_client::datadogV2::api::api_container_images::ListContainerImagesOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ContainerImagesAPI::with_config(configuration);
    let resp = api
        .list_container_images(ListContainerImagesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
