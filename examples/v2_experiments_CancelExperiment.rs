// Cancel experiment returns "The experiment was canceled and unlinked from its
// feature flag allocations." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCancelExperimentV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCancelExperimentV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCancelExperimentV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsCancelExperimentV2RequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment" in the system
    let experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_DATA_ID").unwrap()).expect("Invalid UUID");
    let body =
        ExperimentsCancelExperimentV2Request::new(ExperimentsCancelExperimentV2RequestData::new(
            ExperimentsCancelExperimentV2RequestDataAttributes::new(
                "Cancel the test experiment".to_string(),
            ),
            ExperimentsCancelExperimentV2RequestDataType::CANCEL_EXPERIMENT_REQUEST,
        ));
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .cancel_experiment(experiment_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
