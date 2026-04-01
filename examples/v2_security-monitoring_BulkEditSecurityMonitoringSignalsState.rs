// Bulk update triage state of security signals returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalArchiveReason;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalState;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalStateUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalType;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkStateUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkStateUpdateRequest;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalsBulkStateUpdateRequest::new(vec![
        SecurityMonitoringSignalsBulkStateUpdateData::new(
            SecurityMonitoringSignalStateUpdateAttributes::new(SecurityMonitoringSignalState::OPEN)
                .archive_reason(SecurityMonitoringSignalArchiveReason::NONE),
            "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA".to_string(),
        )
        .type_(SecurityMonitoringSignalType::SIGNAL),
    ]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.bulk_edit_security_monitoring_signals_state(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
