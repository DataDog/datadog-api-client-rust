// List tenancy products returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api.list_tenancy_products("productKeys".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
