// Refresh experiment results returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::RefreshExperimentResultsOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .refresh_experiment_results(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            RefreshExperimentResultsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
