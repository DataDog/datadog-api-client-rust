// Update Custom Ruleset returns "Successfully updated" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::Argument;
use datadog_api_client::datadogV2::model::CustomRule;
use datadog_api_client::datadogV2::model::CustomRuleRevision;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributes;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesCategory;
use datadog_api_client::datadogV2::model::CustomRuleRevisionAttributesSeverity;
use datadog_api_client::datadogV2::model::CustomRuleRevisionDataType;
use datadog_api_client::datadogV2::model::CustomRuleRevisionTest;
use datadog_api_client::datadogV2::model::CustomRulesetDataType;
use datadog_api_client::datadogV2::model::CustomRulesetRequest;
use datadog_api_client::datadogV2::model::CustomRulesetRequestData;
use datadog_api_client::datadogV2::model::CustomRulesetRequestDataAttributes;
use datadog_api_client::datadogV2::model::Language;

#[tokio::main]
async fn main() {
    let body = CustomRulesetRequest::new().data(
        CustomRulesetRequestData::new()
            .attributes(CustomRulesetRequestDataAttributes::new().rules(Some(
                vec![CustomRule::new(
                    DateTime::parse_from_rfc3339("2026-01-09T13:00:57.473141+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                    "foobarbaz".to_string(),
                    CustomRuleRevision::new(
                        CustomRuleRevisionAttributes::new(
                            vec![Argument::new(
                                "YXJndW1lbnQgZGVzY3JpcHRpb24=".to_string(),
                                "YXJndW1lbnRfbmFtZQ==".to_string(),
                            )],
                            CustomRuleRevisionAttributesCategory::SECURITY,
                            "8a66c4e4e631099ad71be3c1ea3ea8fc2d57193e56db2c296e2dd8a508b26b99"
                                .to_string(),
                            "Y29uZHVjdG9yOgogICAgLSBkZXBsb3lfb25seTogdHJ1ZQ==".to_string(),
                            DateTime::parse_from_rfc3339("2026-01-09T13:00:57.473141+00:00")
                                .expect("Failed to parse datetime")
                                .with_timezone(&Utc),
                            "foobarbaz".to_string(),
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
                        ),
                        "revision-123".to_string(),
                        CustomRuleRevisionDataType::CUSTOM_RULE_REVISION,
                    ),
                    "my-rule".to_string(),
                )],
            )))
            .type_(CustomRulesetDataType::CUSTOM_RULESET),
    );
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
