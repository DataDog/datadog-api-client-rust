// Bulk update security signals returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalArchiveReason;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalState;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalType;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringTriageUser;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalsBulkUpdateRequest::new(vec![
        SecurityMonitoringSignalsBulkUpdateData::new(
            SecurityMonitoringSignalUpdateAttributes::new()
                .archive_reason(SecurityMonitoringSignalArchiveReason::NONE)
                .assignee(
                    SecurityMonitoringTriageUser::new(
                        "773b045d-ccf8-4808-bd3b-955ef6a8c940".to_string(),
                    )
                    .name(None),
                )
                .state(SecurityMonitoringSignalState::OPEN),
            "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA".to_string(),
        )
        .type_(SecurityMonitoringSignalType::SIGNAL),
    ]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.bulk_edit_security_monitoring_signals(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
