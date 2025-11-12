// List team connections returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_team_connections::ListTeamConnectionsOptionalParams;
use datadog_api_client::datadogV2::api_team_connections::TeamConnectionsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListTeamConnections", true);
    let api = TeamConnectionsAPI::with_config(configuration);
    let response =
        api.list_team_connections_with_pagination(ListTeamConnectionsOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
