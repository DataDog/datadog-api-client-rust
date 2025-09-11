// Create datastore returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreRequest;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreRequestData;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreRequestDataAttributes;
use datadog_api_client::datadogV2::model::DatastoreDataType;

#[tokio::main]
async fn main() {
    let body = CreateAppsDatastoreRequest::new().data(
        CreateAppsDatastoreRequestData::new(DatastoreDataType::DATASTORES).attributes(
            CreateAppsDatastoreRequestDataAttributes::new(
                "datastore-name".to_string(),
                "primaryKey".to_string(),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api.create_datastore(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
