// Submit metrics with compression returns "Payload accepted" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        MetricPayload::new(
            vec![
                MetricSeries::new(
                    "system.load.1".to_string(),
                    vec![MetricPoint::new().timestamp(1636629071).value(0.7 as f64)],
                ).type_(MetricIntakeType::UNSPECIFIED)
            ],
        );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp =
        api
            .submit_metrics(
                body,
                SubmitMetricsOptionalParams::default().content_encoding(MetricContentEncoding::ZSTD1),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
