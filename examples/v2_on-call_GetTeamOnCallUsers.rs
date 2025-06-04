// Get team on-call users returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::GetTeamOnCallUsersOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    // there are valid "routing_rules" in the system
    let routing_rules_data_id = std::env::var("ROUTING_RULES_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .get_team_on_call_users(
            routing_rules_data_id.clone(),
            GetTeamOnCallUsersOptionalParams::default()
                .include("responders,escalations.responders".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
