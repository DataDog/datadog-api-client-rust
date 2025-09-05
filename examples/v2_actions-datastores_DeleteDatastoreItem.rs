// Delete datastore item returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::DatastoreItemsDataType;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequest;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequestData;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = DeleteAppsDatastoreItemRequest::new().data(
        DeleteAppsDatastoreItemRequestData::new(DatastoreItemsDataType::ITEMS).attributes(
            DeleteAppsDatastoreItemRequestDataAttributes::new("test-key".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .delete_datastore_item(datastore_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
