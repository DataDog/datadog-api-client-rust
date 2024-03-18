// Revoke shared dashboard invitations returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SharedDashboardInvites::new(SharedDashboardInvitesData::SharedDashboardInvitesDataList(
            vec![SharedDashboardInvitesDataObject::new(
                SharedDashboardInvitesDataObjectAttributes::new()
                    .email("test@datadoghq.com".to_string()),
                DashboardInviteType::PUBLIC_DASHBOARD_INVITATION,
            )],
        ));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api
        .delete_public_dashboard_invitation("token".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
