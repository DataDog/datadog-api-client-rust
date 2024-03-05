// Submit distribution points returns "Payload accepted" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    UNIX_EPOCH,
    Duration,
    SystemTime,
};

#[tokio::main]
async fn main() {
    let body =
        DistributionPointsPayload::new(
            vec![
                DistributionPointsSeries::new(
                    "system.load.1.dist".to_string(),
                    vec![
                        vec![
                            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                            Box::new(vec![1.0, 2.0])
                        ]
                    ],
                )
            ],
        );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.submit_distribution_points(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
