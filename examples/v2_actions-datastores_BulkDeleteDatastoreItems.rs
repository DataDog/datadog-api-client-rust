// Bulk delete datastore items returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::BulkDeleteAppsDatastoreItemsRequest;
use datadog_api_client::datadogV2::model::BulkDeleteAppsDatastoreItemsRequestData;
use datadog_api_client::datadogV2::model::BulkDeleteAppsDatastoreItemsRequestDataAttributes;
use datadog_api_client::datadogV2::model::BulkDeleteAppsDatastoreItemsRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = BulkDeleteAppsDatastoreItemsRequest::new().data(
        BulkDeleteAppsDatastoreItemsRequestData::new(
            BulkDeleteAppsDatastoreItemsRequestDataType::ITEMS,
        )
        .attributes(
            BulkDeleteAppsDatastoreItemsRequestDataAttributes::new()
                .item_keys(vec!["test-key".to_string()]),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .bulk_delete_datastore_items(datastore_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
