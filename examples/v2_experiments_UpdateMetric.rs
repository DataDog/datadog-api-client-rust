// Update metric returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsUpdateMetricV2Request;
use datadog_api_client::datadogV2::model::ExperimentsUpdateMetricV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsUpdateMetricV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::MetricType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_metric" in the system
    let experiment_metric_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_METRIC_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = ExperimentsUpdateMetricV2Request::new(
        ExperimentsUpdateMetricV2RequestData::new(MetricType::METRICS)
            .attributes(
                ExperimentsUpdateMetricV2RequestDataAttributes::new()
                    .name("ex-14bb9543f523edde updated".to_string()),
            )
            .id(experiment_metric_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .update_metric(experiment_metric_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
