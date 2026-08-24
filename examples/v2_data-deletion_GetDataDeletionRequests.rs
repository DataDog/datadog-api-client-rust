// Gets a list of data deletion requests returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_deletion::DataDeletionAPI;
use datadog_api_client::datadogV2::api_data_deletion::GetDataDeletionRequestsOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DataDeletionAPI::with_config(configuration);
    let resp = api
        .get_data_deletion_requests(GetDataDeletionRequestsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
