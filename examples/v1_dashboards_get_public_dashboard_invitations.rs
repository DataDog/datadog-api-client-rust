// Get all invitations for a shared dashboard returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "shared_dashboard" in the system
    let shared_dashboard_token = std::env::var("SHARED_DASHBOARD_TOKEN").unwrap();
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp =
        api
            .get_public_dashboard_invitations(
                shared_dashboard_token,
                GetPublicDashboardInvitationsOptionalParams::default(),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
