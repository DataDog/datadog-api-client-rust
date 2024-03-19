// Get all teams returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::ListTeamsOptionalParams;
use datadog_api_client::datadogV2::api::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api.list_teams(ListTeamsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
