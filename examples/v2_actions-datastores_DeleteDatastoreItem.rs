// Delete datastore item returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequest;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequestData;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequestDataAttributes;
use datadog_api_client::datadogV2::model::DeleteAppsDatastoreItemRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();

    // there are valid "datastore items" in the system
    let datastore_items_data_0_id = std::env::var("DATASTORE_ITEMS_DATA_0_ID").unwrap();
    let body = DeleteAppsDatastoreItemRequest::new().data(
        DeleteAppsDatastoreItemRequestData::new(DeleteAppsDatastoreItemRequestDataType::ITEMS)
            .attributes(
                DeleteAppsDatastoreItemRequestDataAttributes::new("test-key".to_string())
                    .id(datastore_items_data_0_id.clone()),
            )
            .id(datastore_data_id.clone()),
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
