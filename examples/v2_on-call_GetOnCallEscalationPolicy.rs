// Get on call escalation policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::GetOnCallEscalationPolicyOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    // there is a valid "escalation_policy" in the system
    let escalation_policy_data_id = std::env::var("ESCALATION_POLICY_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .get_on_call_escalation_policy(
            escalation_policy_data_id.clone(),
            GetOnCallEscalationPolicyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
