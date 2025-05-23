// Get On-Call team routing rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::GetOnCallTeamRoutingRulesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .get_on_call_team_routing_rules(
            "27590dae-47be-4a7d-9abf-8f4e45124020".to_string(),
            GetOnCallTeamRoutingRulesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
