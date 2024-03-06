// Get All Containers returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_containers::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ContainersAPI::with_config(configuration);
    let resp = api
        .list_containers(ListContainersOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}