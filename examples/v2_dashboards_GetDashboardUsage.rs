// Get usage stats for a dashboard returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboards::DashboardsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard" in the system
    let dashboard_id = std::env::var("DASHBOARD_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetDashboardUsage", true);
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.get_dashboard_usage(dashboard_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
