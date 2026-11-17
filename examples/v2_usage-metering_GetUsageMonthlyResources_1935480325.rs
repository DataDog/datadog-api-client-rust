// Get monthly usage by resource supports non-host resources
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::GetUsageMonthlyResourcesOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::MonthlyResourcesView;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_monthly_resources(
            "indexed_points".to_string(),
            "2026-04".to_string(),
            MonthlyResourcesView::ALL,
            GetUsageMonthlyResourcesOptionalParams::default().page_limit(1),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
