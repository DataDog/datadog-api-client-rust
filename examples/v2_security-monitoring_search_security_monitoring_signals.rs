// Get a list of security signals returns "OK" response
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
                    .from(
                        (Utc.with_ymd_and_hms(2019, 1, 2, 9, 42, 36).unwrap() +
                            chrono::Duration::microseconds(320000)).to_rfc3339(),
                    )
                    .query("security:attack status:high".to_string())
                    .to(
                        (Utc.with_ymd_and_hms(2019, 1, 3, 9, 42, 36).unwrap() +
                            chrono::Duration::microseconds(320000)).to_rfc3339(),
                    ),
            )
            .page(
                SecurityMonitoringSignalListRequestPage::new()
                    .cursor(
                        "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ==".to_string(),
                    )
                    .limit(25),
            )
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
