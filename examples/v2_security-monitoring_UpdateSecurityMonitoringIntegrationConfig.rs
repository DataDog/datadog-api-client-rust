// Update an entity context sync configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigResourceType;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringIntegrationConfigUpdateRequest::new(
        SecurityMonitoringIntegrationConfigUpdateData::new(
            SecurityMonitoringIntegrationConfigUpdateAttributes::new()
                .domain("siem-test.com".to_string())
                .enabled(true)
                .integration_type(SecurityMonitoringIntegrationType::GOOGLE_WORKSPACE)
                .name("My GWS Integration (renamed)".to_string())
                .secrets(BTreeMap::from([(
                    "admin_email".to_string(),
                    Value::from("test@example.com"),
                )]))
                .settings(BTreeMap::from([(
                    "setting1".to_string(),
                    Value::from("value1"),
                )])),
            SecurityMonitoringIntegrationConfigResourceType::INTEGRATION_CONFIG,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.UpdateSecurityMonitoringIntegrationConfig", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_monitoring_integration_config("integration_config_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
