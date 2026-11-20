// Update an OpenAI LLM Evaluations account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_aillm_evaluations_integration::OpenAILLMEvaluationsIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsLLMCompletionsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAILLMEvaluationsProjectCloudCostDataIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAILLMEvaluationsIntegrationAccountUpdateRequest::new(
            OpenAILLMEvaluationsIntegrationAccountUpdateData::new(
                OpenAILLMEvaluationsIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        OpenAILLMEvaluationsIntegrationAccountAuthenticationUpdate
                        ::OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthUpdate::new(
                                    OpenAILLMEvaluationsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        OpenAILLMEvaluationsIntegrationDataflowsRequest::new()
                            .openai_llm_completions(
                                OpenAILLMEvaluationsLLMCompletionsIntegrationDataflowRequest::new().enabled(true),
                            )
                            .openai_project_cloud_cost_data(
                                OpenAILLMEvaluationsProjectCloudCostDataIntegrationDataflowRequest
                                ::new().enabled(true),
                            ),
                    )
                    .name("openai-llm-evaluations-acme".to_string())
                    .settings(
                        OpenAILLMEvaluationsIntegrationAccountSettingsUpdate
                        ::new().custom_tags("team:ai,env:prod".to_string()),
                    ),
                "3f8b2c1d-6e4a-4b9f-8c2d-1a7e5b9c0f63".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.update_open_aillm_evaluations_integration_account",
        true,
    );
    let api = OpenAILLMEvaluationsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_open_aillm_evaluations_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
