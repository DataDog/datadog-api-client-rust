// Remove a user from a team returns "No Content" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .delete_team_membership(dd_team_data_id.clone(), "user_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
