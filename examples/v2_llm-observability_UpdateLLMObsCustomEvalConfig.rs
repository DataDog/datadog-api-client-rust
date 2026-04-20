// Create or update a custom evaluator configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigAssessmentCriteria;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigBedrockOptions;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigEvalScope;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigInferenceParams;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigIntegrationProvider;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigLLMJudgeConfig;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigLLMProvider;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigParsingType;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigPromptContent;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigPromptContentValue;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigPromptMessage;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigPromptToolCall;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigPromptToolResult;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigTarget;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigType;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigUpdateData;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigUpdateRequest;
use datadog_api_client::datadogV2::model::LLMObsCustomEvalConfigVertexAIOptions;

#[tokio::main]
async fn main() {
    let body = LLMObsCustomEvalConfigUpdateRequest::new(
        LLMObsCustomEvalConfigUpdateData::new(
            LLMObsCustomEvalConfigUpdateAttributes::new(
                LLMObsCustomEvalConfigTarget::new("my-llm-app".to_string(), true)
                    .eval_scope(LLMObsCustomEvalConfigEvalScope::SPAN)
                    .filter(Some("@service:my-service".to_string()))
                    .root_spans_only(Some(true))
                    .sampling_percentage(Some(50.0 as f64)),
            )
            .category("Custom".to_string())
            .eval_name("my-custom-evaluator".to_string())
            .llm_judge_config(
                LLMObsCustomEvalConfigLLMJudgeConfig::new(
                    LLMObsCustomEvalConfigInferenceParams::new()
                        .frequency_penalty(0.0 as f64)
                        .max_tokens(1024)
                        .presence_penalty(0.0 as f64)
                        .temperature(0.7 as f64)
                        .top_k(50)
                        .top_p(1.0 as f64),
                )
                .assessment_criteria(
                    LLMObsCustomEvalConfigAssessmentCriteria::new()
                        .max_threshold(Some(1.0 as f64))
                        .min_threshold(Some(0.7 as f64))
                        .pass_values(Some(vec!["pass".to_string(), "yes".to_string()]))
                        .pass_when(Some(true)),
                )
                .last_used_library_prompt_template_name(Some("sentiment-analysis-v1".to_string()))
                .modified_library_prompt_template(Some(false))
                .output_schema(None)
                .parsing_type(LLMObsCustomEvalConfigParsingType::STRUCTURED_OUTPUT)
                .prompt_template(vec![LLMObsCustomEvalConfigPromptMessage::new(
                    "user".to_string(),
                )
                .content("Rate the quality of the following response:".to_string())
                .contents(vec![LLMObsCustomEvalConfigPromptContent::new(
                    "text".to_string(),
                    LLMObsCustomEvalConfigPromptContentValue::new()
                        .text("What is the sentiment of this review?".to_string())
                        .tool_call(
                            LLMObsCustomEvalConfigPromptToolCall::new()
                                .arguments(r#"{"location": "San Francisco"}"#.to_string())
                                .id("call_abc123".to_string())
                                .name("get_weather".to_string())
                                .type_("function".to_string()),
                        )
                        .tool_call_result(
                            LLMObsCustomEvalConfigPromptToolResult::new()
                                .name("get_weather".to_string())
                                .result("sunny, 72F".to_string())
                                .tool_id("call_abc123".to_string())
                                .type_("function".to_string()),
                        ),
                )])]),
            )
            .llm_provider(
                LLMObsCustomEvalConfigLLMProvider::new()
                    .bedrock(
                        LLMObsCustomEvalConfigBedrockOptions::new().region("us-east-1".to_string()),
                    )
                    .integration_account_id("my-account-id".to_string())
                    .integration_provider(LLMObsCustomEvalConfigIntegrationProvider::OPENAI)
                    .model_name("gpt-4o".to_string())
                    .vertex_ai(
                        LLMObsCustomEvalConfigVertexAIOptions::new()
                            .location("us-central1".to_string())
                            .project("my-gcp-project".to_string()),
                    ),
            ),
            LLMObsCustomEvalConfigType::EVALUATOR_CONFIG,
        )
        .id("my-custom-evaluator".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsCustomEvalConfig", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_custom_eval_config("eval_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
