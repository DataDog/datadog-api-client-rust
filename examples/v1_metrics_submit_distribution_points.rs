// Submit distribution points returns "Payload accepted" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_metrics::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        DistributionPointsPayload::new(
            vec![
                DistributionPointsSeries::new(
                    "system.load.1.dist".to_string(),
                    vec![vec![(Utc::now()).timestamp() as f64, vec![1.0, 2.0]]],
                )
            ],
        );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.submit_distribution_points(body, SubmitDistributionPointsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
