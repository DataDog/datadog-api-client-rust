// Update an entity context sync configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigResourceType;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationTypeGoogleWorkspace;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringIntegrationConfigUpdateRequest::new(
            SecurityMonitoringIntegrationConfigUpdateData::new(
                SecurityMonitoringIntegrationConfigUpdateAttributes
                ::SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes(
                    Box::new(
                        SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes::new(
                            SecurityMonitoringIntegrationTypeGoogleWorkspace::GOOGLE_WORKSPACE,
                        )
                            .domain("siem-test.com".to_string())
                            .enabled(true)
                            .name("My GWS Integration (renamed)".to_string())
                            .secrets(
                                SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets::new(
                                    SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount::new(
                                        "svc@my-project.iam.gserviceaccount.com".to_string(),
                                        r#"-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----"#.to_string(),
                                        "my-project".to_string(),
                                        "service_account".to_string(),
                                    ),
                                ).admin_email("admin@example.com".to_string()),
                            )
                            .settings(BTreeMap::from([("setting1".to_string(), Value::from("value1"))])),
                    ),
                ),
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
