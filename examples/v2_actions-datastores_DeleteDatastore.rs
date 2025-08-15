// Delete datastore returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;

#[tokio::main]
async fn main() {
    // a "datastore" is created in the system
    let created_datastore_data_id = std::env::var("CREATED_DATASTORE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .delete_datastore(created_datastore_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
