// List Cloud Cost Management Azure configs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_cost_management::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.list_cost_azure_uc_configs().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}