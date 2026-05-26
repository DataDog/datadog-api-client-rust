// Create an AI custom ruleset returns "Successfully created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::AiCustomRulesetDataType;
use datadog_api_client::datadogV2::model::AiCustomRulesetRequest;
use datadog_api_client::datadogV2::model::AiCustomRulesetRequestAttributes;
use datadog_api_client::datadogV2::model::AiCustomRulesetRequestData;

#[tokio::main]
async fn main() {
    let body = AiCustomRulesetRequest::new().data(
        AiCustomRulesetRequestData::new()
            .attributes(AiCustomRulesetRequestAttributes::new(
                "Ruleset description".to_string(),
                "my-ai-ruleset".to_string(),
                "Ruleset short description".to_string(),
            ))
            .id("my-ai-ruleset".to_string())
            .type_(AiCustomRulesetDataType::AI_RULESET),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAiCustomRuleset", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api.create_ai_custom_ruleset(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
