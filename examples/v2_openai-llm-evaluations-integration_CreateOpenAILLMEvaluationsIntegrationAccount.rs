// Create an OpenAI LLM Evaluations account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_aillm_evaluations_integration::OpenAILLMEvaluationsIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsLLMCompletionsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsProjectCloudCostDataIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAILLMEvaluationsIntegrationAccountCreateRequest::new(
            OpenAILLMEvaluationsIntegrationAccountCreateData::new(
                OpenAILLMEvaluationsIntegrationAccountCreateAttributes::new(
                    OpenAILLMEvaluationsIntegrationAccountAuthenticationRequest
                    ::OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthRequest::new(
                                OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "openai-llm-evaluations-acme".to_string(),
                    OpenAILLMEvaluationsIntegrationAccountSettingsRequest
                    ::new().custom_tags("team:ai,env:prod".to_string()),
                ).dataflows(
                    OpenAILLMEvaluationsIntegrationDataflowsRequest::new()
                        .openai_llm_completions(
                            OpenAILLMEvaluationsLLMCompletionsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .openai_project_cloud_cost_data(
                            OpenAILLMEvaluationsProjectCloudCostDataIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.create_open_aillm_evaluations_integration_account",
        true,
    );
    let api = OpenAILLMEvaluationsIntegrationAPI::with_config(configuration);
    let resp = api
        .create_open_aillm_evaluations_integration_account(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
