// Delete team notification rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();

    // there is a valid "team_notification_rule" in the system
    let team_notification_rule_data_id = std::env::var("TEAM_NOTIFICATION_RULE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .delete_team_notification_rule(
            dd_team_data_id.clone(),
            team_notification_rule_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
