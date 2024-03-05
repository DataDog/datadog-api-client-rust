// List all GCP integrations returns "OK" response

use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_gcp_integration::*;



#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = GCPIntegrationAPI::with_config(configuration);
    let resp = api.list_gcp_integration().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
