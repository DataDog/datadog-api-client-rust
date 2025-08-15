// Create datastore from import returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreFromImportRequest;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreFromImportRequestData;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreFromImportRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateAppsDatastoreFromImportRequestDataType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CreateAppsDatastoreFromImportRequest::new().data(
        CreateAppsDatastoreFromImportRequestData::new(
            CreateAppsDatastoreFromImportRequestDataType::DATASTORES,
        )
        .attributes(CreateAppsDatastoreFromImportRequestDataAttributes::new(
            "datastore-name".to_string(),
            "primaryKey".to_string(),
            vec![
                BTreeMap::from([("primaryKey".to_string(), Value::from("key1"))]),
                BTreeMap::from([("primaryKey".to_string(), Value::from("key2"))]),
            ],
        )),
    );
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api.create_datastore_from_import(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
