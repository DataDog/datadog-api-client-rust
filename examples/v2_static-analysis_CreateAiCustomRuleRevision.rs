// Create an AI custom rule revision returns "Successfully created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::AiCustomRuleRevisionDataType;
use datadog_api_client::datadogV2::model::AiCustomRuleRevisionExecutionMode;
use datadog_api_client::datadogV2::model::AiCustomRuleRevisionRequest;
use datadog_api_client::datadogV2::model::AiCustomRuleRevisionRequestAttributes;
use datadog_api_client::datadogV2::model::AiCustomRuleRevisionRequestData;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesCategory;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesSeverity;

#[tokio::main]
async fn main() {
    let body = AiCustomRuleRevisionRequest::new().data(
        AiCustomRuleRevisionRequestData::new()
            .attributes(
                AiCustomRuleRevisionRequestAttributes::new(
                    CustomRuleRevisionAttributesCategory::SECURITY,
                    "Content".to_string(),
                    "Ruleset description".to_string(),
                    vec![],
                    AiCustomRuleRevisionExecutionMode::AUTO,
                    vec!["**/*.py".to_string()],
                    false,
                    false,
                    CustomRuleRevisionAttributesSeverity::ERROR,
                    "Ruleset short description".to_string(),
                )
                .cwe(Some("79".to_string()))
                .version_id(1),
            )
            .id("revision-abc-123".to_string())
            .type_(AiCustomRuleRevisionDataType::AI_RULE_REVISION),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAiCustomRuleRevision", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .create_ai_custom_rule_revision("my-ai-ruleset".to_string(), "my-ai-rule".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
