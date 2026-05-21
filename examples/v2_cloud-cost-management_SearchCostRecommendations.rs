// Search cost recommendations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::SearchCostRecommendationsOptionalParams;
use datadog_api_client::datadogV2::model::RecommendationsFilterRequest;
use datadog_api_client::datadogV2::model::RecommendationsFilterRequestSortItems;

#[tokio::main]
async fn main() {
    let body = RecommendationsFilterRequest::new()
        .filter("@resource_table:aws_ec2_instance".to_string())
        .sort(vec![RecommendationsFilterRequestSortItems::new()
            .expression("potential_daily_savings.amount".to_string())
            .order("DESC".to_string())]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchCostRecommendations", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .search_cost_recommendations(body, SearchCostRecommendationsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
