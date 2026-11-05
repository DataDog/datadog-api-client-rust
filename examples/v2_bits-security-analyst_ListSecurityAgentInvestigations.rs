// List Bits Security Analyst investigations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::api_bits_security_analyst::ListSecurityAgentInvestigationsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_security_agent_investigations", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api
        .list_security_agent_investigations(
            ListSecurityAgentInvestigationsOptionalParams::default().page_size(1),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
