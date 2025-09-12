// Reorder rulesets returns "Successfully reordered rulesets" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::ReorderRulesetResourceArray;
use datadog_api_client::datadogV2::model::ReorderRulesetResourceData;
use datadog_api_client::datadogV2::model::ReorderRulesetResourceDataType;

#[tokio::main]
async fn main() {
    let body = ReorderRulesetResourceArray::new(vec![ReorderRulesetResourceData::new(
        ReorderRulesetResourceDataType::RULESET,
    )]);
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.reorder_rulesets(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
