// Create a team link returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = TeamLinkCreateRequest::new(TeamLinkCreate::new(
        TeamLinkAttributes::new("Link label".to_string(), "https://example.com".to_string())
            .position(0),
        TeamLinkType::TEAM_LINKS,
    ));
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api.create_team_link(dd_team_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}