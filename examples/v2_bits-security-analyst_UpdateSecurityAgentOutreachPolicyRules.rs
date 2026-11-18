// Update outreach policy rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachRuleUpdate;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachRulesUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachRulesUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachRulesUpdateRequestData;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachRulesUpdateType;

#[tokio::main]
async fn main() {
    let body = SecurityAgentOutreachRulesUpdateRequest::new(
        SecurityAgentOutreachRulesUpdateRequestData::new(
            SecurityAgentOutreachRulesUpdateAttributes::new(vec![
                SecurityAgentOutreachRuleUpdate::new(true, "abc-def-ghi".to_string()),
            ]),
            SecurityAgentOutreachRulesUpdateType::OUTREACH_POLICY_RULES_UPDATE,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_security_agent_outreach_policy_rules", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api.update_security_agent_outreach_policy_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
