// Get a Bits Security Analyst investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::api_bits_security_analyst::GetSecurityAgentInvestigationOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_security_agent_investigation", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api
        .get_security_agent_investigation(
            "investigation_id".to_string(),
            GetSecurityAgentInvestigationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
