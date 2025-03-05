// Get a WAF exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;

#[tokio::main]
async fn main() {
    // there is a valid "exclusion_filter" in the system
    let exclusion_filter_data_id = std::env::var("EXCLUSION_FILTER_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .get_application_security_waf_exclusion_filter(exclusion_filter_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
