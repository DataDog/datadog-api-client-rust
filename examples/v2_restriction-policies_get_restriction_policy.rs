// Get a restriction policy returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_restriction_policies::RestrictionPoliciesAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = RestrictionPoliciesAPI::with_config(configuration);
    let resp = api
        .get_restriction_policy("dashboard:test-get".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
