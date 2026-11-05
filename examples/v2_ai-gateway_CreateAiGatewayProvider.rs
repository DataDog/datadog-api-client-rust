// Create an AI Gateway provider returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayCustomProviderSupportedOperation;
use datadog_api_client::datadogV2::model::AiGatewayKnownProvider;
use datadog_api_client::datadogV2::model::AiGatewayModelInput;
use datadog_api_client::datadogV2::model::AiGatewayProviderCreateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayProviderCreateData;
use datadog_api_client::datadogV2::model::AiGatewayProviderCreateRequest;
use datadog_api_client::datadogV2::model::AiGatewayProviderInitialBinding;
use datadog_api_client::datadogV2::model::AiGatewayProviderInitialBindingRelation;
use datadog_api_client::datadogV2::model::AiGatewayProviderIntegrationConfig;
use datadog_api_client::datadogV2::model::AiGatewayProviderType;
use datadog_api_client::datadogV2::model::AiGatewaySupportedOperation;

#[tokio::main]
async fn main() {
    let body =
        AiGatewayProviderCreateRequest::new(
            AiGatewayProviderCreateData::new(
                AiGatewayProviderCreateAttributes::new("production-us".to_string(), "prod-us".to_string())
                    .cost_multiplier(1.0 as f64)
                    .initial_bindings(
                        vec![
                            AiGatewayProviderInitialBinding::new(
                                vec!["org:11111111-1111-1111-1111-111111111111".to_string()],
                                AiGatewayProviderInitialBindingRelation::VIEWER,
                            )
                        ],
                    )
                    .integration_account_id("account-123".to_string())
                    .integration_config(
                        AiGatewayProviderIntegrationConfig::new()
                            .bedrock_region("us-east-1".to_string())
                            .foundry_project("my-project".to_string())
                            .foundry_resource_id(
                                "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/ai/providers/Microsoft.CognitiveServices/accounts/my-foundry".to_string(),
                            )
                            .location("us-central1".to_string())
                            .project_id("my-gcp-project".to_string())
                            .supported_operations(
                                vec![AiGatewayCustomProviderSupportedOperation::OPENAI_CHAT_COMPLETIONS],
                            ),
                    )
                    .known_provider(AiGatewayKnownProvider::OPENAI)
                    .models(
                        vec![
                            AiGatewayModelInput::new(
                                "gpt-5.2-codex".to_string(),
                                "gpt-5.2-codex-20260801".to_string(),
                            ).supported_operations(vec![AiGatewaySupportedOperation::OPENAI_CHAT_COMPLETIONS])
                        ],
                    ),
                AiGatewayProviderType::PROVIDERS,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_ai_gateway_provider", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.create_ai_gateway_provider(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
