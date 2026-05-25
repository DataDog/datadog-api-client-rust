// Update an AI custom ruleset returns "Successfully updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::AiCustomRulesetDataType;
use datadog_api_client::datadogV2::model::AiCustomRulesetUpdateAttributes;
use datadog_api_client::datadogV2::model::AiCustomRulesetUpdateData;
use datadog_api_client::datadogV2::model::AiCustomRulesetUpdateRequest;

#[tokio::main]
async fn main() {
    let body = AiCustomRulesetUpdateRequest::new().data(
        AiCustomRulesetUpdateData::new()
            .attributes(
                AiCustomRulesetUpdateAttributes::new()
                    .description("Ruleset description".to_string())
                    .name("my-ai-ruleset".to_string())
                    .short_description("Ruleset short description".to_string()),
            )
            .id("my-ai-ruleset".to_string())
            .type_(AiCustomRulesetDataType::AI_RULESET),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateAiCustomRuleset", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .update_ai_custom_ruleset("my-ai-ruleset".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
