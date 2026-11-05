// Count Bits Security Analyst investigations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::api_bits_security_analyst::CountSecurityAgentInvestigationsOptionalParams;
use datadog_api_client::datadogV2::model::SecurityAgentInvestigationCountField;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.count_security_agent_investigations", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api
        .count_security_agent_investigations(
            SecurityAgentInvestigationCountField::VERDICT,
            CountSecurityAgentInvestigationsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
