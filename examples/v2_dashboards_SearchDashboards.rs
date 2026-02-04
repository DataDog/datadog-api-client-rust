// Search dashboards returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV2::api_dashboards::SearchDashboardsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchDashboards", true);
    let api = DashboardsAPI::with_config(configuration);
    let resp = api
        .search_dashboards(SearchDashboardsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
