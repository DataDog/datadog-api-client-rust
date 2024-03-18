// Create a security filter returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SecurityFilterCreateRequest::new(SecurityFilterCreateData::new(
        SecurityFilterCreateAttributes::new(
            vec![SecurityFilterExclusionFilter::new(
                "Exclude staging".to_string(),
                "source:staging".to_string(),
            )],
            SecurityFilterFilteredDataType::LOGS,
            true,
            "Example-Security-Monitoring".to_string(),
            "service:ExampleSecurityMonitoring".to_string(),
        ),
        SecurityFilterType::SECURITY_FILTERS,
    ));
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_filter(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
