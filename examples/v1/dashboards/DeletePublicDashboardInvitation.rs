// Revoke shared dashboard invitations returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        SharedDashboardInvites::new(
            SharedDashboardInvitesData::SharedDashboardInvitesDataList(
                Box::new(
                    vec![
                        SharedDashboardInvitesDataObject::new(
                            SharedDashboardInvitesDataObjectAttributes::new().email("test@datadoghq.com".to_string()),
                            DashboardInviteType::PUBLIC_DASHBOARD_INVITATION,
                        )
                    ],
                ),
            ),
        );
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.delete_public_dashboard_invitation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
