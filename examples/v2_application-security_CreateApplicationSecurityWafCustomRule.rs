// Create a WAF custom rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleAction;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleActionAction;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleActionParameters;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleCondition;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInput;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInputAddress;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOperator;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOptions;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleConditionParameters;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleCreateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleCreateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleCreateRequest;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleTags;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleTagsCategory;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafCustomRuleType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        ApplicationSecurityWafCustomRuleCreateRequest::new(
            ApplicationSecurityWafCustomRuleCreateData::new(
                ApplicationSecurityWafCustomRuleCreateAttributes::new(
                    false,
                    vec![
                        ApplicationSecurityWafCustomRuleCondition::new(
                            ApplicationSecurityWafCustomRuleConditionOperator::MATCH_REGEX,
                            ApplicationSecurityWafCustomRuleConditionParameters::new(
                                vec![
                                    ApplicationSecurityWafCustomRuleConditionInput::new(
                                        ApplicationSecurityWafCustomRuleConditionInputAddress::SERVER_DB_STATEMENT,
                                    ).key_path(vec![])
                                ],
                            )
                                .data("blocked_users".to_string())
                                .list(vec![])
                                .options(
                                    ApplicationSecurityWafCustomRuleConditionOptions::new()
                                        .case_sensitive(false)
                                        .min_length(0),
                                )
                                .regex("path.*".to_string())
                                .value("custom_tag".to_string()),
                        )
                    ],
                    false,
                    "Block request from a bad useragent".to_string(),
                    ApplicationSecurityWafCustomRuleTags::new(
                        ApplicationSecurityWafCustomRuleTagsCategory::BUSINESS_LOGIC,
                        "users.login.success".to_string(),
                    ).additional_properties(BTreeMap::from([])),
                )
                    .action(
                        ApplicationSecurityWafCustomRuleAction::new()
                            .action(ApplicationSecurityWafCustomRuleActionAction::BLOCK_REQUEST)
                            .parameters(
                                ApplicationSecurityWafCustomRuleActionParameters::new()
                                    .location("/blocking".to_string())
                                    .status_code(403),
                            ),
                    )
                    .path_glob("/api/search/*".to_string())
                    .scope(
                        vec![
                            ApplicationSecurityWafCustomRuleScope::new(
                                "prod".to_string(),
                                "billing-service".to_string(),
                            )
                        ],
                    ),
                ApplicationSecurityWafCustomRuleType::CUSTOM_RULE,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api.create_application_security_waf_custom_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
