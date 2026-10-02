// Update experiment metric group returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2Request;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2RequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment" in the system
    let experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_DATA_ID").unwrap()).expect("Invalid UUID");

    // there is a valid "experiment_metric_group" in the system
    let experiment_metric_group_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_METRIC_GROUP_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = ExperimentsPatchExperimentMetricGroupV2Request::new(
        ExperimentsPatchExperimentMetricGroupV2RequestData::new(
            ExperimentsPatchExperimentMetricGroupV2RequestDataType::EXPERIMENT_METRIC_GROUPS,
        )
        .attributes(
            ExperimentsPatchExperimentMetricGroupV2RequestDataAttributes::new()
                .name("ex-14bb9543f523edde updated".to_string()),
        )
        .id(experiment_metric_group_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .update_experiment_metric_group(
            experiment_data_id.clone(),
            experiment_metric_group_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
