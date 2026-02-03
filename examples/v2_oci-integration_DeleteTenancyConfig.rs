// Delete tenancy config returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api.delete_tenancy_config("tenancy_ocid".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
