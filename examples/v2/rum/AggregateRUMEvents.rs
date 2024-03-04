// Aggregate RUM events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_rum::RUMAPI;
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
        RUMAggregateRequest::new()
            .compute(
                vec![
                    RUMCompute::new(RUMAggregationFunction::PERCENTILE_90)
                        .metric("@view.time_spent".to_string())
                        .type_(RUMComputeType::TOTAL)
                ],
            )
            .filter(
                RUMQueryFilter::new()
                    .from("now-15m".to_string())
                    .query("@type:view AND @session.type:user".to_string())
                    .to("now".to_string()),
            )
            .group_by(
                vec![
                    RUMGroupBy::new("@view.time_spent".to_string())
                        .limit(10)
                        .total(RUMGroupByTotal::RUMGroupByTotalBoolean(false))
                ],
            )
            .options(RUMQueryOptions::new().timezone("GMT".to_string()))
            .page(RUMQueryPageOptions::new().limit(25));
    let configuration = Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api.aggregate_rum_events(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
