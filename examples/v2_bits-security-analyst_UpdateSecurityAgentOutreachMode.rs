// Update the outreach mode returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachMode;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachModeAttributes;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachModeUpdateData;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachModeUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityAgentOutreachPolicyType;

#[tokio::main]
async fn main() {
    let body =
        SecurityAgentOutreachModeUpdateRequest::new(SecurityAgentOutreachModeUpdateData::new(
            SecurityAgentOutreachModeAttributes::new(SecurityAgentOutreachMode::AUTOMATED),
            SecurityAgentOutreachPolicyType::OUTREACH_POLICY,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_security_agent_outreach_mode", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api.update_security_agent_outreach_mode(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
