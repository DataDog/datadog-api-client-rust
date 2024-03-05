// Get a list of security signals returns "OK" response with pagination
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringSignalListRequest::new()
            .filter(
                SecurityMonitoringSignalListRequestFilter::new()
                    .from(((Utc::now() + chrono::Duration::minutes(-15)).to_rfc3339()).to_rfc3339())
                    .query("security:attack status:high".to_string())
                    .to(((Utc::now()).to_rfc3339()).to_rfc3339()),
            )
            .page(SecurityMonitoringSignalListRequestPage::new().limit(2))
            .sort(SecurityMonitoringSignalsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp =
        api
            .search_security_monitoring_signals(SearchSecurityMonitoringSignalsOptionalParams::default().body(body))
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
