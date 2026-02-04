// List connection groups returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::api_action_connection::ListConnectionGroupsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListConnectionGroups", true);
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .list_connection_groups(ListConnectionGroupsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
