// Tag Configuration Cardinality Estimator returns "Success" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .estimate_metrics_output_series(
            "system.cpu.idle".to_string(),
            EstimateMetricsOutputSeriesOptionalParams::default()
                .filter_groups("app,host".to_string())
                .filter_num_aggregations(4),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
