// Bulk put datastore items returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequest;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequestData;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequestDataAttributes;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequestDataType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = BulkPutAppsDatastoreItemsRequest::new().data(
        BulkPutAppsDatastoreItemsRequestData::new(BulkPutAppsDatastoreItemsRequestDataType::ITEMS)
            .attributes(BulkPutAppsDatastoreItemsRequestDataAttributes::new(vec![
                BTreeMap::from([(
                    "28173b88-1a0e-001e-28c0-7664b6410518".to_string(),
                    Value::from("key1"),
                )]),
                BTreeMap::from([(
                    "28173b88-1a0e-001e-28c0-7664b6410518".to_string(),
                    Value::from("key2"),
                )]),
            ])),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .bulk_put_datastore_items(datastore_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
