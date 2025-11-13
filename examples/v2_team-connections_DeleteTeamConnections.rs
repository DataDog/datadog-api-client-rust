// Delete team connections returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_team_connections::TeamConnectionsAPI;
use datadog_api_client::datadogV2::model::TeamConnectionDeleteRequest;
use datadog_api_client::datadogV2::model::TeamConnectionDeleteRequestDataItem;
use datadog_api_client::datadogV2::model::TeamConnectionType;

#[tokio::main]
async fn main() {
    let body = TeamConnectionDeleteRequest::new(vec![TeamConnectionDeleteRequestDataItem::new(
        "12345678-1234-5678-9abc-123456789012".to_string(),
        TeamConnectionType::TEAM_CONNECTION,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteTeamConnections", true);
    let api = TeamConnectionsAPI::with_config(configuration);
    let resp = api.delete_team_connections(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
