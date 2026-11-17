// Add an allowed outreach domain returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachDomainAddRequest;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachDomainAddRequestData;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachDomainAddRequestDataAttributes;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachDomainAddType;

#[tokio::main]
async fn main() {
    let body =
        SecurityAgentOutreachDomainAddRequest::new(SecurityAgentOutreachDomainAddRequestData::new(
            SecurityAgentOutreachDomainAddRequestDataAttributes::new("acme.com".to_string()),
            SecurityAgentOutreachDomainAddType::OUTREACH_POLICY_DOMAIN_ADD,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.add_security_agent_outreach_policy_domain", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api.add_security_agent_outreach_policy_domain(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
