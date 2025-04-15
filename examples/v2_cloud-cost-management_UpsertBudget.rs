// Update if exists, or create a new budget returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::BudgetWithEntries;
use datadog_api_client::datadogV2::model::BudgetWithEntriesData;

#[tokio::main]
async fn main() {
    let body = BudgetWithEntries::new().data(BudgetWithEntriesData::new().id("1".to_string()));
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.upsert_budget(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
