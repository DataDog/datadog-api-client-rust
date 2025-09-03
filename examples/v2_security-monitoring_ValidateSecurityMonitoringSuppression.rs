// Validate a suppression rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSuppressionType;
use datadog_api_client::datadogV2::model::SecurityMonitoringSuppressionUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringSuppressionUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringSuppressionUpdateRequest;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringSuppressionUpdateRequest::new(
        SecurityMonitoringSuppressionUpdateData::new(
            SecurityMonitoringSuppressionUpdateAttributes::new()
                .data_exclusion_query("source:cloudtrail account_id:12345".to_string())
                .description(
                    "This rule suppresses low-severity signals in staging environments."
                        .to_string(),
                )
                .enabled(true)
                .name("Custom suppression".to_string())
                .rule_query("type:log_detection source:cloudtrail".to_string()),
            SecurityMonitoringSuppressionType::SUPPRESSIONS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.validate_security_monitoring_suppression(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
