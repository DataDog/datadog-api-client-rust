// Create an OpenAI Codex integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_codex_integration::OpenAICodexIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAICodexDoraIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAICodexIntegrationAccountCreateRequest::new(
            OpenAICodexIntegrationAccountCreateData::new(
                OpenAICodexIntegrationAccountCreateAttributes::new(
                    OpenAICodexIntegrationAccountAuthenticationRequest
                    ::OpenAICodexIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            OpenAICodexIntegrationAccountBearerTokenAuthRequest::new(
                                OpenAICodexIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "openai-codex-acme".to_string(),
                    OpenAICodexIntegrationAccountSettingsRequest::new("your-workspace-id".to_string()),
                ).dataflows(
                    OpenAICodexIntegrationDataflowsRequest::new()
                        .openai_codex_dora(OpenAICodexDoraIntegrationDataflowRequest::new().enabled(true))
                        .openai_codex_turn_metrics(
                            OpenAICodexTurnMetricsIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_open_ai_codex_integration_account", true);
    let api = OpenAICodexIntegrationAPI::with_config(configuration);
    let resp = api.create_open_ai_codex_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
