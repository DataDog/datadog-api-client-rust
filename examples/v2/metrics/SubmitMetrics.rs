// Submit metrics returns "Payload accepted" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        MetricPayload::new(
            vec![
                MetricSeries::new(
                    "system.load.1".to_string(),
                    vec![
                        MetricPoint::new()
                            .timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
                            .value(0.7)
                    ],
                )
                    .resources(vec![MetricResource::new().name("dummyhost".to_string()).type_("host".to_string())])
                    .type_(MetricIntakeType::UNSPECIFIED)
            ],
        );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.submit_metrics(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
