// List shared dashboards for a dashboard returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboard_sharing::DashboardSharingAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListSharedDashboardsByDashboardId", true);
    let api = DashboardSharingAPI::with_config(configuration);
    let resp = api
        .list_shared_dashboards_by_dashboard_id("abc-def-ghi".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
