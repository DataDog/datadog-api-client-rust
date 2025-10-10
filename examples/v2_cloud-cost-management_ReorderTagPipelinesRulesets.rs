// Reorder tag pipeline rulesets returns "Successfully reordered rulesets" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::ReorderRulesetResourceArray;
use datadog_api_client::datadogV2::model::ReorderRulesetResourceData;
use datadog_api_client::datadogV2::model::ReorderRulesetResourceDataType;

#[tokio::main]
async fn main() {
    let body = ReorderRulesetResourceArray::new(vec![
        ReorderRulesetResourceData::new(ReorderRulesetResourceDataType::RULESET)
            .id("55ef2385-9ae1-4410-90c4-5ac1b60fec10".to_string()),
        ReorderRulesetResourceData::new(ReorderRulesetResourceDataType::RULESET)
            .id("a7b8c9d0-1234-5678-9abc-def012345678".to_string()),
        ReorderRulesetResourceData::new(ReorderRulesetResourceDataType::RULESET)
            .id("f1e2d3c4-b5a6-9780-1234-567890abcdef".to_string()),
    ]);
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.reorder_tag_pipelines_rulesets(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
