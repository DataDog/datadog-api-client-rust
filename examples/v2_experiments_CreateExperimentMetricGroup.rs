// Create experiment metric group returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentMetricGroupV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentMetricGroupV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentMetricGroupV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentMetricGroupV2RequestDataAttributesMetricsItems;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2RequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment" in the system
    let experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_DATA_ID").unwrap()).expect("Invalid UUID");

    // there is a valid "experiment_metric" in the system
    let experiment_metric_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_METRIC_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = ExperimentsCreateExperimentMetricGroupV2Request::new(
        ExperimentsCreateExperimentMetricGroupV2RequestData::new(
            ExperimentsCreateExperimentMetricGroupV2RequestDataAttributes::new(
                "ex-14bb9543f523edde".to_string(),
            )
            .metrics(vec![
                ExperimentsCreateExperimentMetricGroupV2RequestDataAttributesMetricsItems::new(
                    experiment_metric_data_id.clone(),
                ),
            ]),
            ExperimentsPatchExperimentMetricGroupV2RequestDataType::EXPERIMENT_METRIC_GROUPS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .create_experiment_metric_group(experiment_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
