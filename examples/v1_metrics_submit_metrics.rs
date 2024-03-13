// Submit metrics returns "Payload accepted" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_metrics::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body = MetricsPayload::new(vec![Series::new(
        "system.load.1".to_string(),
        vec![vec![Some(1636629071 as f64), Some(1.1 as f64)]],
    )
    .tags(vec!["test:ExampleMetric".to_string()])
    .type_("gauge".to_string())]);
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .submit_metrics(body, SubmitMetricsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}