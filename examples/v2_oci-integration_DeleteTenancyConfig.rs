// Delete tenancy config returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "oci_tenancy" resource in the system
    let oci_tenancy_data_id = std::env::var("OCI_TENANCY_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api.delete_tenancy_config(oci_tenancy_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
