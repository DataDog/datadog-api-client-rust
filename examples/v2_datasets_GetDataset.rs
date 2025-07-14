// Get a single dataset by ID returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_datasets::DatasetsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DatasetsAPI::with_config(configuration);
    let resp = api.get_dataset("dataset_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
