// Update a connection group returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ConnectionGroupDataAttributesRequest;
use datadog_api_client::datadogV2::model::ConnectionGroupDataRequest;
use datadog_api_client::datadogV2::model::ConnectionGroupType;
use datadog_api_client::datadogV2::model::UpdateConnectionGroupRequest;

#[tokio::main]
async fn main() {
    let body = UpdateConnectionGroupRequest::new(
        ConnectionGroupDataRequest::new(ConnectionGroupType::CONNECTION_GROUP).attributes(
            ConnectionGroupDataAttributesRequest::new()
                .connections(vec![])
                .description("An updated test connection group for AWS integrations".to_string())
                .name("My Connection Group Updated".to_string())
                .tag_keys(vec![]),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateConnectionGroup", true);
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .update_connection_group("connection_group_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
