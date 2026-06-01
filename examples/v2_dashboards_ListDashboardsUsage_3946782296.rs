// Get usage stats for all dashboards with viewed_before filter returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV2::api_dashboards::ListDashboardsUsageOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListDashboardsUsage", true);
    let api = DashboardsAPI::with_config(configuration);
    let resp = api
        .list_dashboards_usage(
            ListDashboardsUsageOptionalParams::default()
                .filter_viewed_before("2025-04-26T00:00:00Z".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
