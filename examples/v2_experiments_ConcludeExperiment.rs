// Conclude experiment returns "The experiment was concluded and the winning
// variant was rolled out to its linked feature flag allocation." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsConcludeExperimentV2Request;
use datadog_api_client::datadogV2::model::ExperimentsConcludeExperimentV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsConcludeExperimentV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsConcludeExperimentV2RequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ExperimentsConcludeExperimentV2Request::new(
        ExperimentsConcludeExperimentV2RequestData::new(
            ExperimentsConcludeExperimentV2RequestDataAttributes::new("treatment".to_string()),
            ExperimentsConcludeExperimentV2RequestDataType::CONCLUDE_EXPERIMENT_REQUEST,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .conclude_experiment(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
