// Get bandit returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_bandit", true);
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .get_bandit(Uuid::parse_str("4b1f3c2e-8d5a-4f6b-9c7e-1a2b3c4d5e6f").expect("invalid UUID"))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
