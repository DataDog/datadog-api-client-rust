// Create a shared dashboard returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard" in the system
    let dashboard_id = std::env::var("DASHBOARD_ID").unwrap();
    let body = SharedDashboard::new(dashboard_id.clone(), DashboardType::CUSTOM_TIMEBOARD)
        .global_time(
            DashboardGlobalTime::new().live_span(DashboardGlobalTimeLiveSpan::PAST_ONE_HOUR),
        )
        .share_type(Some(DashboardShareType::OPEN));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_public_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}