// Put datastore item returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::PutAppsDatastoreItemRequest;
use datadog_api_client::datadogV2::model::PutAppsDatastoreItemRequestData;
use datadog_api_client::datadogV2::model::PutAppsDatastoreItemRequestDataAttributes;
use datadog_api_client::datadogV2::model::PutAppsDatastoreItemRequestDataType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = PutAppsDatastoreItemRequest::new().data(
        PutAppsDatastoreItemRequestData::new(PutAppsDatastoreItemRequestDataType::ITEMS)
            .attributes(PutAppsDatastoreItemRequestDataAttributes::new(
                BTreeMap::from([
                    (
                        "28173b88-1a0e-001e-28c0-7664b6410518".to_string(),
                        Value::from("new-item-key"),
                    ),
                    ("data".to_string(), Value::from("example data")),
                    ("key".to_string(), Value::from("value")),
                ]),
            ))
            .id("e7e64418-b60c-4789-9612-895ac8423207".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .put_datastore_item(datastore_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
