// Update an OpenAI Codex integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_codex_integration::OpenAICodexIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAICodexDoraIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::OpenAICodexIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAICodexIntegrationAccountUpdateRequest::new(
            OpenAICodexIntegrationAccountUpdateData::new(
                OpenAICodexIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        OpenAICodexIntegrationAccountAuthenticationUpdate
                        ::OpenAICodexIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                OpenAICodexIntegrationAccountBearerTokenAuthUpdate::new(
                                    OpenAICodexIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        OpenAICodexIntegrationDataflowsRequest::new()
                            .openai_codex_dora(OpenAICodexDoraIntegrationDataflowRequest::new().enabled(true))
                            .openai_codex_turn_metrics(
                                OpenAICodexTurnMetricsIntegrationDataflowRequest::new().enabled(true),
                            ),
                    )
                    .name("openai-codex-acme".to_string())
                    .settings(
                        OpenAICodexIntegrationAccountSettingsUpdate
                        ::new().codex_workspace_id("your-workspace-id".to_string()),
                    ),
                "7d2f9c4e-3a1b-4c8e-9f6d-2b5a8e1c0d47".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_open_ai_codex_integration_account", true);
    let api = OpenAICodexIntegrationAPI::with_config(configuration);
    let resp = api
        .update_open_ai_codex_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
