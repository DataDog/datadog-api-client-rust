// Update a shared dashboard returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "shared_dashboard" in the system
    let shared_dashboard_token = std::env::var("SHARED_DASHBOARD_TOKEN").unwrap();
    let body =
        SharedDashboardUpdateRequest::new(
            Some(
                SharedDashboardUpdateRequestGlobalTime
                ::new().live_span(DashboardGlobalTimeLiveSpan::PAST_FIFTEEN_MINUTES),
            ),
        )
            .share_list(Some(vec![]))
            .share_type(Some(DashboardShareType::OPEN));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.update_public_dashboard(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
