// Validate entity context sync credentials returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationConfigResourceType;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationCredentialsValidateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationCredentialsValidateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationCredentialsValidateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationTypeGoogleWorkspace;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringIntegrationCredentialsValidateRequest::new(
            SecurityMonitoringIntegrationCredentialsValidateData::new(
                SecurityMonitoringIntegrationCredentialsValidateAttributes
                ::SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes(
                    Box::new(
                        SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes::new(
                            "siem-test.com".to_string(),
                            SecurityMonitoringIntegrationTypeGoogleWorkspace::GOOGLE_WORKSPACE,
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
                        ),
                    ),
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
