// Get a list of all escalation policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::ListOnCallEscalationPoliciesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .list_on_call_escalation_policies(ListOnCallEscalationPoliciesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
