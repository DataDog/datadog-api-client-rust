// Run an LLM inference returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnthropicEffort;
use datadog_api_client::datadogV2::model::LLMObsAnthropicMetadata;
use datadog_api_client::datadogV2::model::LLMObsAnthropicThinkingConfig;
use datadog_api_client::datadogV2::model::LLMObsAnthropicThinkingType;
use datadog_api_client::datadogV2::model::LLMObsAzureOpenAIMetadata;
use datadog_api_client::datadogV2::model::LLMObsBedrockMetadata;
use datadog_api_client::datadogV2::model::LLMObsInferenceContent;
use datadog_api_client::datadogV2::model::LLMObsInferenceContentValue;
use datadog_api_client::datadogV2::model::LLMObsInferenceFunction;
use datadog_api_client::datadogV2::model::LLMObsInferenceMessage;
use datadog_api_client::datadogV2::model::LLMObsInferenceTool;
use datadog_api_client::datadogV2::model::LLMObsInferenceToolCall;
use datadog_api_client::datadogV2::model::LLMObsInferenceToolResult;
use datadog_api_client::datadogV2::model::LLMObsIntegrationInferenceRequest;
use datadog_api_client::datadogV2::model::LLMObsIntegrationName;
use datadog_api_client::datadogV2::model::LLMObsOpenAIMetadata;
use datadog_api_client::datadogV2::model::LLMObsOpenAIReasoningEffort;
use datadog_api_client::datadogV2::model::LLMObsOpenAIReasoningSummary;
use datadog_api_client::datadogV2::model::LLMObsVertexAIMetadata;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = LLMObsIntegrationInferenceRequest::new(
        vec![LLMObsInferenceMessage::new()
            .content("What is the capital of France?".to_string())
            .contents(vec![LLMObsInferenceContent::new(
                "text".to_string(),
                LLMObsInferenceContentValue::new()
                    .text("Hello, how can I help you?".to_string())
                    .tool_call(
                        LLMObsInferenceToolCall::new()
                            .arguments(BTreeMap::from([(
                                "location".to_string(),
                                Value::from("San Francisco"),
                            )]))
                            .name("get_weather".to_string())
                            .tool_id("call_abc123".to_string())
                            .type_("function".to_string()),
                    )
                    .tool_call_result(
                        LLMObsInferenceToolResult::new()
                            .name("get_weather".to_string())
                            .result("The weather in San Francisco is 68°F and sunny.".to_string())
                            .tool_id("call_abc123".to_string())
                            .type_("function".to_string()),
                    ),
            )])
            .id("msg_001".to_string())
            .role("user".to_string())
            .tool_calls(vec![LLMObsInferenceToolCall::new()
                .arguments(BTreeMap::from([(
                    "location".to_string(),
                    Value::from("San Francisco"),
                )]))
                .name("get_weather".to_string())
                .tool_id("call_abc123".to_string())
                .type_("function".to_string())])
            .tool_results(vec![LLMObsInferenceToolResult::new()
                .name("get_weather".to_string())
                .result("The weather in San Francisco is 68°F and sunny.".to_string())
                .tool_id("call_abc123".to_string())
                .type_("function".to_string())])],
        "gpt-4o".to_string(),
    )
    .anthropic_metadata(
        LLMObsAnthropicMetadata::new()
            .effort(Some(LLMObsAnthropicEffort::MEDIUM))
            .thinking(
                LLMObsAnthropicThinkingConfig::new(LLMObsAnthropicThinkingType::ENABLED)
                    .budget_tokens(Some(1024)),
            ),
    )
    .azure_openai_metadata(
        LLMObsAzureOpenAIMetadata::new()
            .deployment_id("my-gpt4-deployment".to_string())
            .model_version("0613".to_string())
            .resource_name("my-azure-resource".to_string()),
    )
    .bedrock_metadata(LLMObsBedrockMetadata::new().region("us-east-1".to_string()))
    .frequency_penalty(Some(0.0 as f64))
    .json_schema(Some(
        r#"{"type":"object","properties":{"answer":{"type":"string"}}}"#.to_string(),
    ))
    .max_completion_tokens(Some(1024))
    .max_tokens(Some(1024))
    .openai_metadata(
        LLMObsOpenAIMetadata::new()
            .reasoning_effort(Some(LLMObsOpenAIReasoningEffort::MEDIUM))
            .reasoning_summary(Some(LLMObsOpenAIReasoningSummary::AUTO)),
    )
    .presence_penalty(Some(0.0 as f64))
    .temperature(Some(0.7 as f64))
    .tools(vec![LLMObsInferenceTool::new(
        LLMObsInferenceFunction::new(
            "get_weather".to_string(),
            BTreeMap::from([("type".to_string(), Value::from("object"))]),
        )
        .description("Get the current weather for a location.".to_string()),
        "function".to_string(),
    )])
    .top_k(Some(50))
    .top_p(Some(1.0 as f64))
    .vertex_ai_metadata(
        LLMObsVertexAIMetadata::new()
            .location("us-central1".to_string())
            .project("my-gcp-project".to_string())
            .project_ids(vec!["my-gcp-project".to_string()]),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsIntegrationInference", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_integration_inference(
            LLMObsIntegrationName::OPENAI,
            "account_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
