// Aggregate spans returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_spans::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SpansAggregateRequest::new().data(
        SpansAggregateData::new()
            .attributes(
                SpansAggregateRequestAttributes::new()
                    .compute(vec![SpansCompute::new(SpansAggregationFunction::COUNT)
                        .interval("5m".to_string())
                        .type_(SpansComputeType::TIMESERIES)])
                    .filter(
                        SpansQueryFilter::new()
                            .from("now-15m".to_string())
                            .query("*".to_string())
                            .to("now".to_string()),
                    ),
            )
            .type_(SpansAggregateRequestType::AGGREGATE_REQUEST),
    );
    let configuration = Configuration::new();
    let api = SpansAPI::with_config(configuration);
    let resp = api.aggregate_spans(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
