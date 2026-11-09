// Get a GitHub App Action Connection
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;

#[tokio::main]
async fn main() {
    // there is a valid "github_app_action_connection" in the system
    let github_app_action_connection_data_id =
        std::env::var("GITHUB_APP_ACTION_CONNECTION_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .get_action_connection(github_app_action_connection_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
