// Modify the triage assignee of a security signal returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalAssigneeUpdateRequest::new(
        SecurityMonitoringSignalAssigneeUpdateData::new(
            SecurityMonitoringSignalAssigneeUpdateAttributes::new(
                SecurityMonitoringTriageUser::new("".to_string()),
            ),
        ),
    );
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .edit_security_monitoring_signal_assignee(
            "AQAAAYG1bl5K4HuUewAAAABBWUcxYmw1S0FBQmt2RmhRN0V4ZUVnQUE".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
