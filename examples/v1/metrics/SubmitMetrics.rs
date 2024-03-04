// Submit metrics returns "Payload accepted" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        MetricsPayload::new(
            vec![
                Series::new(
                    "system.load.1".to_string(),
                    vec![vec![SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, 1.1]],
                )
                    .tags(vec!["test:ExampleMetric".to_string()])
                    .type_("gauge".to_string())
            ],
        );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.submit_metrics(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
