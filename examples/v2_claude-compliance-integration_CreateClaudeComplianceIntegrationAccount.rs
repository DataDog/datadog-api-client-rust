// Create a Claude Compliance integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_claude_compliance_integration::ClaudeComplianceIntegrationAPI;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::ClaudeComplianceIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ClaudeComplianceIntegrationAccountCreateRequest::new(
            ClaudeComplianceIntegrationAccountCreateData::new(
                ClaudeComplianceIntegrationAccountCreateAttributes::new(
                    ClaudeComplianceIntegrationAccountAuthenticationRequest
                    ::ClaudeComplianceIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            ClaudeComplianceIntegrationAccountBearerTokenAuthRequest::new(
                                ClaudeComplianceIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "claude-compliance-acme".to_string(),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_claude_compliance_integration_account", true);
    let api = ClaudeComplianceIntegrationAPI::with_config(configuration);
    let resp = api.create_claude_compliance_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
