// Change the related incidents of a security signal returns "OK" response
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
        SecurityMonitoringSignalIncidentsUpdateRequest::new(
            SecurityMonitoringSignalIncidentsUpdateData::new(
                SecurityMonitoringSignalIncidentsUpdateAttributes::new(vec![2066]),
            ),
        );
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.edit_security_monitoring_signal_incidents(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
