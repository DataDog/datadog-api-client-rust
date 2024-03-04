// Modify the triage assignee of a security signal returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringSignalAssigneeUpdateRequest::new(
            SecurityMonitoringSignalAssigneeUpdateData::new(
                SecurityMonitoringSignalAssigneeUpdateAttributes::new(
                    SecurityMonitoringTriageUser::new("".to_string()),
                ),
            ),
        );
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.edit_security_monitoring_signal_assignee(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
