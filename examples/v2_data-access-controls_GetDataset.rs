// Get a Data Access Control dataset by ID returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_access_controls::DataAccessControlsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetDataset", true);
    let api = DataAccessControlsAPI::with_config(configuration);
    let resp = api.get_dataset("dataset_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
