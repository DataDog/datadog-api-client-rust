// Get a list of security signals returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::model::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSignalListRequest::new()
        .filter(
            SecurityMonitoringSignalListRequestFilter::new()
                .from("2021-11-11T10:56:11+00:00".to_string())
                .query("security:attack status:high".to_string())
                .to("2021-11-11T11:11:11+00:00".to_string()),
        )
        .page(SecurityMonitoringSignalListRequestPage::new().limit(2))
        .sort(SecurityMonitoringSignalsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let response = api.search_security_monitoring_signals_with_pagination(
        SearchSecurityMonitoringSignalsOptionalParams::default().body(body),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
