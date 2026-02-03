// Create a security signal investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationRequestAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationRequestData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalInvestigationType;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalInvestigationRequest::new(
        SecurityMonitoringSignalInvestigationRequestData::new(
            SecurityMonitoringSignalInvestigationRequestAttributes::new(
                "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA".to_string(),
            )
            .deployment("live".to_string()),
            SecurityMonitoringSignalInvestigationType::INVESTIGATION_SIGNAL,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSignalInvestigation", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_signal_investigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
