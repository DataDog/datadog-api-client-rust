// Delete a secure embed for a dashboard returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboard_secure_embed::DashboardSecureEmbedAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteDashboardSecureEmbed", true);
    let api = DashboardSecureEmbedAPI::with_config(configuration);
    let resp = api
        .delete_dashboard_secure_embed("dashboard_id".to_string(), "token".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
