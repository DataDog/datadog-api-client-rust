// Create a Bits Security Analyst investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_security_analyst::BitsSecurityAnalystAPI;
use datadog_api_client::datadogV2::model::SecurityAgentInvestigationCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityAgentInvestigationCreateData;
use datadog_api_client::datadogV2::model::SecurityAgentInvestigationCreateRequest;
use datadog_api_client::datadogV2::model::SecurityAgentInvestigationType;

#[tokio::main]
async fn main() {
    let body =
        SecurityAgentInvestigationCreateRequest::new(SecurityAgentInvestigationCreateData::new(
            SecurityAgentInvestigationCreateAttributes::new(
                "AQAAAaGNHjq4XzqcIQAAAABBWm8xY0wybUFBQmNYSjl1QUFBQUFBQUE".to_string(),
            )
            .rerun(false),
            SecurityAgentInvestigationType::INVESTIGATIONS,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_security_agent_investigation", true);
    let api = BitsSecurityAnalystAPI::with_config(configuration);
    let resp = api.create_security_agent_investigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
