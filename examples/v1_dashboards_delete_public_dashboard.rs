// Revoke a shared dashboard URL returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.delete_public_dashboard("token".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
