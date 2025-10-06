// Get a WAF Policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .get_application_security_waf_policy("policy_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
