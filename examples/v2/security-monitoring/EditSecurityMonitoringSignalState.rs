// Change the triage state of a security signal returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringSignalStateUpdateRequest::new(
            SecurityMonitoringSignalStateUpdateData::new(
                SecurityMonitoringSignalStateUpdateAttributes::new(
                    SecurityMonitoringSignalState::OPEN,
                ).archive_reason(SecurityMonitoringSignalArchiveReason::NONE),
            ),
        );
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.edit_security_monitoring_signal_state(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
