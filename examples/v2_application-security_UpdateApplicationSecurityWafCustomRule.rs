// Update a WAF Custom Rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleCondition;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInput;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInputAddress;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOperator;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionParameters;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleTags;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleTagsCategory;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleType;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleUpdateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleUpdateRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "custom_rule" in the system
    let custom_rule_data_id = std::env::var("CUSTOM_RULE_DATA_ID").unwrap();
    let body =
        ApplicationSecurityWafCustomRuleUpdateRequest::new(
            ApplicationSecurityWafCustomRuleUpdateData::new(
                ApplicationSecurityWafCustomRuleUpdateAttributes::new(
                    false,
                    vec![
                        ApplicationSecurityWafCustomRuleCondition::new(
                            ApplicationSecurityWafCustomRuleConditionOperator::MATCH_REGEX,
                            ApplicationSecurityWafCustomRuleConditionParameters::new(
                                vec![
                                    ApplicationSecurityWafCustomRuleConditionInput::new(
                                        ApplicationSecurityWafCustomRuleConditionInputAddress::SERVER_REQUEST_QUERY,
                                    ).key_path(vec!["id".to_string()])
                                ],
                            ).regex("badactor".to_string()),
                        )
                    ],
                    false,
                    "test".to_string(),
                    ApplicationSecurityWafCustomRuleTags::new(
                        ApplicationSecurityWafCustomRuleTagsCategory::ATTACK_ATTEMPT,
                        "test".to_string(),
                    ).additional_properties(BTreeMap::from([])),
                )
                    .path_glob("/test".to_string())
                    .scope(vec![ApplicationSecurityWafCustomRuleScope::new("test".to_string(), "test".to_string())]),
                ApplicationSecurityWafCustomRuleType::CUSTOM_RULE,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .update_application_security_waf_custom_rule(custom_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
