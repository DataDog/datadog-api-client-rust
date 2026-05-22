// Validate entity context sync credentials returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigResourceType;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationCredentialsValidateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationCredentialsValidateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationCredentialsValidateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringIntegrationCredentialsValidateRequest::new(
        SecurityMonitoringIntegrationCredentialsValidateData::new(
            SecurityMonitoringIntegrationCredentialsValidateAttributes::new(
                "siem-test.com".to_string(),
                SecurityMonitoringIntegrationType::GOOGLE_WORKSPACE,
                BTreeMap::from([("admin_email".to_string(), Value::from("test@example.com"))]),
            ),
            SecurityMonitoringIntegrationConfigResourceType::INTEGRATION_CONFIG,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.ValidateSecurityMonitoringIntegrationCredentials",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .validate_security_monitoring_integration_credentials(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
