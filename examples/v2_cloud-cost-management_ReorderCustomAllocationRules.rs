// Reorder custom allocation rules returns "Successfully reordered rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::ReorderRuleResourceArray;
use datadog_api_client::datadogV2::model::ReorderRuleResourceData;
use datadog_api_client::datadogV2::model::ReorderRuleResourceDataType;

#[tokio::main]
async fn main() {
    let body = ReorderRuleResourceArray::new(vec![
        ReorderRuleResourceData::new(ReorderRuleResourceDataType::ARBITRARY_RULE)
            .id("456".to_string()),
        ReorderRuleResourceData::new(ReorderRuleResourceDataType::ARBITRARY_RULE)
            .id("123".to_string()),
        ReorderRuleResourceData::new(ReorderRuleResourceDataType::ARBITRARY_RULE)
            .id("789".to_string()),
    ]);
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.reorder_custom_allocation_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
