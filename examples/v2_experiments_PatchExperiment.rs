// Patch experiment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentV2Request;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment" in the system
    let experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = ExperimentsPatchExperimentV2Request::new(
        ExperimentsPatchExperimentV2RequestData::new(
            ExperimentsPatchExperimentV2ResponseDataType::EXPERIMENTS,
        )
        .attributes(
            ExperimentsPatchExperimentV2RequestDataAttributes::new()
                .name("ex-14bb9543f523edde updated".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.patch_experiment(experiment_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
