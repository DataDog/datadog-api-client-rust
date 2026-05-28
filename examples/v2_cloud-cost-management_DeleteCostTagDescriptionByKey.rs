// Delete a Cloud Cost Management tag description returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::DeleteCostTagDescriptionByKeyOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .delete_cost_tag_description_by_key(
            "tag_key".to_string(),
            DeleteCostTagDescriptionByKeyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
