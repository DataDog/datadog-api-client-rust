// Update metric collection returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsPatchMetricCollectionV2Request;
use datadog_api_client::datadogV2::model::ExperimentsPatchMetricCollectionV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsPatchMetricCollectionV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsPatchMetricCollectionV2RequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_metric_collection" in the system
    let experiment_metric_collection_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_METRIC_COLLECTION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = ExperimentsPatchMetricCollectionV2Request::new(
        ExperimentsPatchMetricCollectionV2RequestData::new(
            ExperimentsPatchMetricCollectionV2RequestDataType::METRIC_COLLECTIONS,
        )
        .attributes(
            ExperimentsPatchMetricCollectionV2RequestDataAttributes::new()
                .name("ex-14bb9543f523edde updated".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .update_metric_collection(experiment_metric_collection_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
