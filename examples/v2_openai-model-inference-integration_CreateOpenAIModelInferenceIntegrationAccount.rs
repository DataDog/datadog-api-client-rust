// Create an OpenAI Model Inference account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_model_inference_integration::OpenAIModelInferenceIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAIModelInferenceIntegrationAccountCreateRequest::new(
            OpenAIModelInferenceIntegrationAccountCreateData::new(
                OpenAIModelInferenceIntegrationAccountCreateAttributes::new(
                    OpenAIModelInferenceIntegrationAccountAuthenticationRequest
                    ::OpenAIModelInferenceIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            OpenAIModelInferenceIntegrationAccountBearerTokenAuthRequest::new(
                                OpenAIModelInferenceIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "openai-model-inference-acme".to_string(),
                    OpenAIModelInferenceIntegrationAccountSettingsRequest
                    ::new().custom_tags("team:ai,env:prod".to_string()),
                ).dataflows(
                    OpenAIModelInferenceIntegrationDataflowsRequest::new()
                        .openai_llm_completions(
                            OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .openai_project_cloud_cost_data(
                            OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.create_open_ai_model_inference_integration_account",
        true,
    );
    let api = OpenAIModelInferenceIntegrationAPI::with_config(configuration);
    let resp = api
        .create_open_ai_model_inference_integration_account(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
