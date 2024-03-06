// Search Audit Logs events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_audit::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        AuditLogsSearchEventsRequest::new()
            .filter(
                AuditLogsQueryFilter::new()
                    .from("now-15m".to_string())
                    .query("@type:session AND @session.type:user".to_string())
                    .to("now".to_string()),
            )
            .options(AuditLogsQueryOptions::new().time_offset(0).timezone("GMT".to_string()))
            .page(AuditLogsQueryPageOptions::new().limit(25))
            .sort(AuditLogsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = AuditAPI::with_config(configuration);
    let resp = api.search_audit_logs(SearchAuditLogsOptionalParams::default().body(body)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
