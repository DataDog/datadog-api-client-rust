// Create an AI custom rule returns "Successfully created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::AiCustomRuleDataType;
use datadog_api_client::datadogV2::model::AiCustomRuleRequest;
use datadog_api_client::datadogV2::model::AiCustomRuleRequestAttributes;
use datadog_api_client::datadogV2::model::AiCustomRuleRequestData;

#[tokio::main]
async fn main() {
    let body = AiCustomRuleRequest::new().data(
        AiCustomRuleRequestData::new()
            .attributes(AiCustomRuleRequestAttributes::new().name("my-ai-rule".to_string()))
            .id("my-ai-rule".to_string())
            .type_(AiCustomRuleDataType::AI_RULE),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAiCustomRule", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .create_ai_custom_rule("my-ai-ruleset".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
