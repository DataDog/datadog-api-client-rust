// Delete an existing Action Connection returns "The resource was deleted
// successfully." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;

#[tokio::main]
async fn main() {
    // there is a valid "action_connection" in the system
    let action_connection_data_id = std::env::var("ACTION_CONNECTION_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .delete_action_connection(action_connection_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
