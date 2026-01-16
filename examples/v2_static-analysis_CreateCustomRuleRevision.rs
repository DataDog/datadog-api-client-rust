// Create Custom Rule Revision returns "Successfully created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::Argument;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesCategory;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesSeverity;
use datadog_api_client::datadogV2::model::CustomRuleRevisionDataType;
use datadog_api_client::datadogV2::model::CustomRuleRevisionInputAttributes;
use datadog_api_client::datadogV2::model::CustomRuleRevisionRequest;
use datadog_api_client::datadogV2::model::CustomRuleRevisionRequestData;
use datadog_api_client::datadogV2::model::CustomRuleRevisionTest;
use datadog_api_client::datadogV2::model::Language;

#[tokio::main]
async fn main() {
    let body = CustomRuleRevisionRequest::new().data(
        CustomRuleRevisionRequestData::new()
            .attributes(CustomRuleRevisionInputAttributes::new(
                vec![Argument::new(
                    "YXJndW1lbnQgZGVzY3JpcHRpb24=".to_string(),
                    "YXJndW1lbnRfbmFtZQ==".to_string(),
                )],
                CustomRuleRevisionAttributesCategory::SECURITY,
                "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                "Initial revision".to_string(),
                Some("CVE-2024-1234".to_string()),
                Some("CWE-79".to_string()),
                "bG9uZyBkZXNjcmlwdGlvbg==".to_string(),
                Some("https://docs.example.com/rules/my-rule".to_string()),
                false,
                false,
                Language::PYTHON,
                CustomRuleRevisionAttributesSeverity::ERROR,
                "c2hvcnQgZGVzY3JpcHRpb24=".to_string(),
                false,
                vec!["security".to_string(), "custom".to_string()],
                vec![CustomRuleRevisionTest::new(
                    1,
                    "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                    "test.yaml".to_string(),
                )],
                "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
            ))
            .type_(CustomRuleRevisionDataType::CUSTOM_RULE_REVISION),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCustomRuleRevision", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .create_custom_rule_revision("ruleset_name".to_string(), "rule_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
