// Get a list of security signals returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    UNIX_EPOCH,
    Duration,
    SystemTime,
};

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringSignalListRequest::new()
            .filter(
                SecurityMonitoringSignalListRequestFilter::new()
                    .from(SystemTime::now().add(Duration::from_secs(-15 * 60)).as_secs() as i64)
                    .query("security:attack status:high".to_string())
                    .to(SystemTime::now().as_secs() as i64),
            )
            .page(SecurityMonitoringSignalListRequestPage::new().limit(2))
            .sort(SecurityMonitoringSignalsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.search_security_monitoring_signals(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
