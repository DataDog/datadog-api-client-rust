// List team connections with filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::ListTeamConnectionsOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .list_team_connections(
            ListTeamConnectionsOptionalParams::default()
                .filter_sources(vec!["github".to_string()])
                .page_size(10),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
