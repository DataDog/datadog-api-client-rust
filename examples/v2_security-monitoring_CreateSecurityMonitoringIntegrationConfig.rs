// Create an entity context sync configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigCreateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigResourceType;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationTypeGoogleWorkspace;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringIntegrationConfigCreateRequest::new(
            SecurityMonitoringIntegrationConfigCreateData::new(
                SecurityMonitoringIntegrationConfigCreateAttributes
                ::SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes(
                    Box::new(
                        SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes::new(
                            "siem-test.com".to_string(),
                            SecurityMonitoringIntegrationTypeGoogleWorkspace::GOOGLE_WORKSPACE,
                            "My GWS Integration".to_string(),
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
                        ).settings(BTreeMap::from([("setting1".to_string(), Value::from("value1"))])),
                    ),
                ),
                SecurityMonitoringIntegrationConfigResourceType::INTEGRATION_CONFIG,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.CreateSecurityMonitoringIntegrationConfig", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .create_security_monitoring_integration_config(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
