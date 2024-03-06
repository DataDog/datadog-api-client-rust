// Get a list of Audit Logs events returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_audit::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = AuditAPI::with_config(configuration);
    let resp = api.list_audit_logs(ListAuditLogsOptionalParams::default().page_limit(2)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
