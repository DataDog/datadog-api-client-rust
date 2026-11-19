// Get bandit results returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::GetBanditResultsOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "bandit" in the system
    let bandit_data_id =
        uuid::Uuid::parse_str(&std::env::var("BANDIT_DATA_ID").unwrap()).expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_bandit", true);
    configuration.set_unstable_operation_enabled("v2.get_bandit_results", true);
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .get_bandit_results(
            bandit_data_id.clone(),
            GetBanditResultsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
