// List all Application Security exclusion filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api.list_application_security_exclusion_filters().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
