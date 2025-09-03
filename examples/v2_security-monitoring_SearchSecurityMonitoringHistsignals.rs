// Search hist signals returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SearchSecurityMonitoringHistsignalsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalListRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalListRequestFilter;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalListRequestPage;
use datadog_api_client::datadogV2::model::SecurityMonitoringSignalsSort;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringSignalListRequest::new()
            .filter(
                SecurityMonitoringSignalListRequestFilter::new()
                    .from(
                        DateTime::parse_from_rfc3339("2019-01-02T09:42:36.320000+00:00")
                            .expect("Failed to parse datetime")
                            .with_timezone(&Utc),
                    )
                    .query("security:attack status:high".to_string())
                    .to(
                        DateTime::parse_from_rfc3339("2019-01-03T09:42:36.320000+00:00")
                            .expect("Failed to parse datetime")
                            .with_timezone(&Utc),
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
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchSecurityMonitoringHistsignals", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .search_security_monitoring_histsignals(
            SearchSecurityMonitoringHistsignalsOptionalParams::default().body(body),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
