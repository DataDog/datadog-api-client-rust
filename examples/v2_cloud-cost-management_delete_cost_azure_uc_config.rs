// Delete Cloud Cost Management Azure config returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api::api_cloud_cost_management::CloudCostManagementAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.delete_cost_azure_uc_config("100".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
