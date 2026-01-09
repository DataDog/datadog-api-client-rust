// Update a critical asset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetSeverity;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetType;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringCriticalAssetUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "critical_asset" in the system
    let critical_asset_data_id = std::env::var("CRITICAL_ASSET_DATA_ID").unwrap();
    let body =
        SecurityMonitoringCriticalAssetUpdateRequest::new(
            SecurityMonitoringCriticalAssetUpdateData::new(
                SecurityMonitoringCriticalAssetUpdateAttributes::new()
                    .enabled(false)
                    .query("no:alert".to_string())
                    .rule_query(
                        "type:(log_detection OR signal_correlation OR workload_security OR application_security) ruleId:djg-ktx-ipq".to_string(),
                    )
                    .severity(SecurityMonitoringCriticalAssetSeverity::DECREASE)
                    .tags(vec!["env:production".to_string()])
                    .version(1),
                SecurityMonitoringCriticalAssetType::CRITICAL_ASSETS,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_monitoring_critical_asset(critical_asset_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
