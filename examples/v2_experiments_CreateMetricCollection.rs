// Create metric collection returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricCollectionV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsPatchMetricCollectionV2RequestDataType;

#[tokio::main]
async fn main() {
    let body = ExperimentsCreateMetricCollectionV2Request::new(
        ExperimentsCreateMetricCollectionV2RequestData::new(
            ExperimentsCreateMetricCollectionV2RequestDataAttributes::new(
                "ex-14bb9543f523edde".to_string(),
            ),
            ExperimentsPatchMetricCollectionV2RequestDataType::METRIC_COLLECTIONS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_metric_collection(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
