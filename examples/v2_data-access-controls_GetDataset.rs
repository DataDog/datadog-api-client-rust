// Get a Data Access Control dataset by ID returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_access_controls::DataAccessControlsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "dataset" in the system
    let dataset_data_id = std::env::var("DATASET_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = DataAccessControlsAPI::with_config(configuration);
    let resp = api.get_dataset(dataset_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
