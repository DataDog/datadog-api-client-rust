// Create a critical asset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetCreateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetSeverity;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetType;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringCriticalAssetCreateRequest::new(
            SecurityMonitoringCriticalAssetCreateData::new(
                SecurityMonitoringCriticalAssetCreateAttributes::new(
                    "host:examplesecuritymonitoring".to_string(),
                    "type:(log_detection OR signal_correlation OR workload_security OR application_security) source:cloudtrail".to_string(),
                    SecurityMonitoringCriticalAssetSeverity::DECREASE,
                ).tags(vec!["team:security".to_string(), "env:test".to_string()]),
                SecurityMonitoringCriticalAssetType::CRITICAL_ASSETS,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_critical_asset(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
