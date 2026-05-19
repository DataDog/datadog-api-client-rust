// Get usage stats for all dashboards returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV2::api_dashboards::ListDashboardsUsageOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListDashboardsUsage", true);
    let api = DashboardsAPI::with_config(configuration);
    let response = api.list_dashboards_usage_with_pagination(
        ListDashboardsUsageOptionalParams::default().page_limit(500),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
