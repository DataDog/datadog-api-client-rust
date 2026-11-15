// Delete a cost recommendation custom rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_cost_recommendation_custom_rule", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .delete_cost_recommendation_custom_rule("custom_rule_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
