// Start experiment returns "The experiment was started." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::StartExperimentOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "configured_experiment" in the system
    let configured_experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("CONFIGURED_EXPERIMENT_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .start_experiment(
            configured_experiment_data_id.clone(),
            StartExperimentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
