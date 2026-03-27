// Get all Data Access Control datasets returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_access_controls::DataAccessControlsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DataAccessControlsAPI::with_config(configuration);
    let resp = api.get_all_datasets().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
