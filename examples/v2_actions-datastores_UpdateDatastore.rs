// Update datastore returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::DatastoreDataType;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreRequest;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreRequestData;
use datadog_api_client::datadogV2::model::UpdateAppsDatastoreRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let body = UpdateAppsDatastoreRequest::new().data(
        UpdateAppsDatastoreRequestData::new(DatastoreDataType::DATASTORES)
            .attributes(
                UpdateAppsDatastoreRequestDataAttributes::new().name("updated name".to_string()),
            )
            .id(datastore_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api.update_datastore(datastore_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
