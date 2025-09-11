// List datastore items returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_actions_datastores::ActionsDatastoresAPI;
use datadog_api_client::datadogV2::api_actions_datastores::ListDatastoreItemsOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "datastore" in the system
    let datastore_data_id = std::env::var("DATASTORE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ActionsDatastoresAPI::with_config(configuration);
    let resp = api
        .list_datastore_items(
            datastore_data_id.clone(),
            ListDatastoreItemsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
