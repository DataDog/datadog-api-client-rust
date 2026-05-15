// List Cloud Cost Management orchestrators returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::ListCostTagMetadataOrchestratorsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListCostTagMetadataOrchestrators", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .list_cost_tag_metadata_orchestrators(
            "filter[month]".to_string(),
            ListCostTagMetadataOrchestratorsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
