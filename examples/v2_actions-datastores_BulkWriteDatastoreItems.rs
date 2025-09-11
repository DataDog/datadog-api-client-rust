// Bulk write datastore items returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequest;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequestData;
use datadog_api_client::datadogV2::model::BulkPutAppsDatastoreItemsRequestDataAttributes;
use datadog_api_client::datadogV2::model::DatastoreItemsDataType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = BulkPutAppsDatastoreItemsRequest::new().data(
        BulkPutAppsDatastoreItemsRequestData::new(DatastoreItemsDataType::ITEMS).attributes(
            BulkPutAppsDatastoreItemsRequestDataAttributes::new(vec![
                BTreeMap::from([
                    ("id".to_string(), Value::from("cust_3141")),
                    ("name".to_string(), Value::from("Johnathan")),
                ]),
                BTreeMap::from([
                    ("id".to_string(), Value::from("cust_3142")),
                    ("name".to_string(), Value::from("Mary")),
                ]),
            ]),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .bulk_write_datastore_items(datastore_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
