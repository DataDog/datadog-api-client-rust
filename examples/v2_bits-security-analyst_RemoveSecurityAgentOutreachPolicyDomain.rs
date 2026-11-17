// Remove an allowed outreach domain returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.remove_security_agent_outreach_policy_domain", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api
        .remove_security_agent_outreach_policy_domain("acme.com".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
