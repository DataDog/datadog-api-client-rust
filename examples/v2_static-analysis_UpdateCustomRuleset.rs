// Update Custom Ruleset returns "Successfully updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::Argument;
use datadog_api_client::datadogV2::model::CustomRule;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesCategory;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesSeverity;
use datadog_api_client::datadogV2::model::CustomRuleRevisionInput;
use datadog_api_client::datadogV2::model::CustomRuleRevisionTest;
use datadog_api_client::datadogV2::model::CustomRulesetDataType;
use datadog_api_client::datadogV2::model::CustomRulesetRequest;
use datadog_api_client::datadogV2::model::CustomRulesetRequestData;
use datadog_api_client::datadogV2::model::CustomRulesetRequestDataAttributes;
use datadog_api_client::datadogV2::model::Language;

#[tokio::main]
async fn main() {
    let body = CustomRulesetRequest::new().data(CustomRulesetRequestData::new(
        CustomRulesetRequestDataAttributes::new("my-ruleset".to_string())
            .description("bG9uZyBkZXNjcmlwdGlvbg==".to_string())
            .rules(Some(vec![CustomRule::new(
                "my-rule".to_string(),
                "my-rule".to_string(),
            )
            .last_revision(
                CustomRuleRevisionInput::new()
                    .arguments(Some(vec![Argument::new(
                        "YXJndW1lbnQgZGVzY3JpcHRpb24=".to_string(),
                        "YXJndW1lbnRfbmFtZQ==".to_string(),
                    )]))
                    .category(CustomRuleRevisionAttributesCategory::SECURITY)
                    .code("Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string())
                    .creation_message("Initial revision".to_string())
                    .cve(Some("CVE-2024-1234".to_string()))
                    .cwe(Some("CWE-79".to_string()))
                    .description("bG9uZyBkZXNjcmlwdGlvbg==".to_string())
                    .documentation_url(Some("https://docs.example.com/rules/my-rule".to_string()))
                    .is_published(false)
                    .is_testing(false)
                    .language(Language::PYTHON)
                    .severity(CustomRuleRevisionAttributesSeverity::ERROR)
                    .short_description("c2hvcnQgZGVzY3JpcHRpb24=".to_string())
                    .should_use_ai_fix(false)
                    .tags(Some(vec!["security".to_string(), "custom".to_string()]))
                    .tests(Some(vec![CustomRuleRevisionTest::new(
                        1,
                        "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                        "test.yaml".to_string(),
                    )]))
                    .tree_sitter_query(
                        "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                    ),
            )
            .revisions(Some(vec![CustomRuleRevisionInput::new()
                .arguments(Some(vec![Argument::new(
                    "YXJndW1lbnQgZGVzY3JpcHRpb24=".to_string(),
                    "YXJndW1lbnRfbmFtZQ==".to_string(),
                )]))
                .category(CustomRuleRevisionAttributesCategory::SECURITY)
                .code("Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string())
                .creation_message("Initial revision".to_string())
                .cve(Some("CVE-2024-1234".to_string()))
                .cwe(Some("CWE-79".to_string()))
                .description("bG9uZyBkZXNjcmlwdGlvbg==".to_string())
                .documentation_url(Some("https://docs.example.com/rules/my-rule".to_string()))
                .is_published(false)
                .is_testing(false)
                .language(Language::PYTHON)
                .severity(CustomRuleRevisionAttributesSeverity::ERROR)
                .short_description("c2hvcnQgZGVzY3JpcHRpb24=".to_string())
                .should_use_ai_fix(false)
                .tags(Some(vec!["security".to_string(), "custom".to_string()]))
                .tests(Some(vec![CustomRuleRevisionTest::new(
                    1,
                    "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                    "test.yaml".to_string(),
                )]))
                .tree_sitter_query(
                    "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                )]))]))
            .short_description("c2hvcnQgZGVzY3JpcHRpb24=".to_string()),
        "my-ruleset".to_string(),
        CustomRulesetDataType::CUSTOM_RULESET,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateCustomRuleset", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .update_custom_ruleset("ruleset_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
