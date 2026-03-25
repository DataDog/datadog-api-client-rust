// Bulk update triage assignee of security signals returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalType;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkAssigneeUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkAssigneeUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsBulkAssigneeUpdateRequest;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalsBulkAssigneeUpdateRequest::new(vec![
        SecurityMonitoringSignalsBulkAssigneeUpdateData::new(
            SecurityMonitoringSignalsBulkAssigneeUpdateAttributes::new(
                "773b045d-ccf8-4808-bd3b-955ef6a8c940".to_string(),
            ),
            "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA".to_string(),
        )
        .type_(SecurityMonitoringSignalType::SIGNAL),
    ]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .bulk_edit_security_monitoring_signals_assignee(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
