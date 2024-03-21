// Get All Container groups returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_containers::ContainersAPI;
use datadog_api_client::datadogV2::api::api_containers::ListContainersOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ContainersAPI::with_config(configuration);
    let resp = api
        .list_containers(
            ListContainersOptionalParams::default().group_by("short_image".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
