// Get the Cloud Cost Management billing currency returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::GetCostTagMetadataCurrencyOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCostTagMetadataCurrency", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .get_cost_tag_metadata_currency(
            "filter[month]".to_string(),
            GetCostTagMetadataCurrencyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
