// Update datastore item returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreItemRequest;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreItemRequestData;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreItemRequestDataAttributes;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreItemRequestDataAttributesItemChanges;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreItemRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = UpdateAppsDatastoreItemRequest::new().data(
        UpdateAppsDatastoreItemRequestData::new(UpdateAppsDatastoreItemRequestDataType::ITEMS)
            .attributes(UpdateAppsDatastoreItemRequestDataAttributes::new(
                UpdateAppsDatastoreItemRequestDataAttributesItemChanges::new(),
                "test-key".to_string(),
            )),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .update_datastore_item(datastore_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
