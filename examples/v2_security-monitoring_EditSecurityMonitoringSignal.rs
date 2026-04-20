// Update security signal triage state or assignee returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalArchiveReason;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalMetadataType;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalState;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringTriageUser;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalUpdateRequest::new(
        SecurityMonitoringSignalUpdateData::new(
            SecurityMonitoringSignalUpdateAttributes::new()
                .archive_reason(SecurityMonitoringSignalArchiveReason::NONE)
                .assignee(
                    SecurityMonitoringTriageUser::new(
                        "773b045d-ccf8-4808-bd3b-955ef6a8c940".to_string(),
                    )
                    .name(None),
                )
                .state(SecurityMonitoringSignalState::OPEN),
        )
        .type_(SecurityMonitoringSignalMetadataType::SIGNAL_METADATA),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .edit_security_monitoring_signal("signal_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
