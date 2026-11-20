// Update a Claude Compliance integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_claude_compliance_integration::ClaudeComplianceIntegrationAPI;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ClaudeComplianceIntegrationAccountUpdateRequest::new(
            ClaudeComplianceIntegrationAccountUpdateData::new(
                ClaudeComplianceIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        ClaudeComplianceIntegrationAccountAuthenticationUpdate
                        ::ClaudeComplianceIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                ClaudeComplianceIntegrationAccountBearerTokenAuthUpdate::new(
                                    ClaudeComplianceIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .name("claude-compliance-acme".to_string()),
                "5f1a8d3c-7e2b-4f9a-b6c4-8d0e3a7f1b52".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_claude_compliance_integration_account", true);
    let api = ClaudeComplianceIntegrationAPI::with_config(configuration);
    let resp = api
        .update_claude_compliance_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
