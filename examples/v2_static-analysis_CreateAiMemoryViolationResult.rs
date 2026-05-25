// Create an AI memory violation result returns "Successfully created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::AiMemoryViolationResultDataType;
use datadog_api_client::datadogV2::model::AiMemoryViolationResultRequest;
use datadog_api_client::datadogV2::model::AiMemoryViolationResultRequestAttributes;
use datadog_api_client::datadogV2::model::AiMemoryViolationResultRequestData;
use datadog_api_client::datadogV2::model::AiMemoryViolationType;

#[tokio::main]
async fn main() {
    let body = AiMemoryViolationResultRequest::new().data(
        AiMemoryViolationResultRequestData::new()
            .attributes(AiMemoryViolationResultRequestAttributes::new(
                10,
                "This is a false positive because the input is sanitized.".to_string(),
                "src/main.py".to_string(),
                "my-repo".to_string(),
                "my-ai-ruleset/my-ai-rule".to_string(),
                "abc123def456789012345678901234567890abcd".to_string(),
                AiMemoryViolationType::FP,
            ))
            .id("violation-abc".to_string())
            .type_(AiMemoryViolationResultDataType::AI_MEMORY_VIOLATION_RESULT),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAiMemoryViolationResult", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api.create_ai_memory_violation_result(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
