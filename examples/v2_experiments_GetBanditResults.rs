// Get bandit results returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::GetBanditResultsOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_bandit_results", true);
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .get_bandit_results(
            Uuid::parse_str("4b1f3c2e-8d5a-4f6b-9c7e-1a2b3c4d5e6f").expect("invalid UUID"),
            GetBanditResultsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
