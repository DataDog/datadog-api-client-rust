// Update an OpenAI Model Inference account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_model_inference_integration::OpenAIModelInferenceIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAIModelInferenceIntegrationAccountUpdateRequest::new(
            OpenAIModelInferenceIntegrationAccountUpdateData::new(
                OpenAIModelInferenceIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        OpenAIModelInferenceIntegrationAccountAuthenticationUpdate
                        ::OpenAIModelInferenceIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                OpenAIModelInferenceIntegrationAccountBearerTokenAuthUpdate::new(
                                    OpenAIModelInferenceIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        OpenAIModelInferenceIntegrationDataflowsRequest::new()
                            .openai_llm_completions(
                                OpenAIModelInferenceLLMCompletionsIntegrationDataflowRequest::new().enabled(true),
                            )
                            .openai_project_cloud_cost_data(
                                OpenAIModelInferenceProjectCloudCostDataIntegrationDataflowRequest
                                ::new().enabled(true),
                            ),
                    )
                    .name("openai-model-inference-acme".to_string())
                    .settings(
                        OpenAIModelInferenceIntegrationAccountSettingsUpdate
                        ::new().custom_tags("team:ai,env:prod".to_string()),
                    ),
                "3f8b2c1d-6e4a-4b9f-8c2d-1a7e5b9c0f63".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.update_open_ai_model_inference_integration_account",
        true,
    );
    let api = OpenAIModelInferenceIntegrationAPI::with_config(configuration);
    let resp = api
        .update_open_ai_model_inference_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
