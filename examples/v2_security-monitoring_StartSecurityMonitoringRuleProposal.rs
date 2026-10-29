// Start a rule proposal returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleProposalInputType;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleProposalStartRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleProposalStartRequestAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleProposalStartRequestData;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleProposalType;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringRuleProposalStartRequest::new(
            SecurityMonitoringRuleProposalStartRequestData::new(
                SecurityMonitoringRuleProposalStartRequestAttributes::new(
                    "Detect repeated failed Okta logins followed by a successful login from the same IP.".to_string(),
                    SecurityMonitoringRuleProposalInputType::NATURAL_LANGUAGE,
                ),
                SecurityMonitoringRuleProposalType::RULE_PROPOSALS,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.start_security_monitoring_rule_proposal", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.start_security_monitoring_rule_proposal(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
