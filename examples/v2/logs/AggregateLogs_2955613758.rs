// Aggregate compute events with group by returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::LogsAPI;
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
        LogsAggregateRequest::new()
            .compute(
                vec![
                    LogsCompute::new(LogsAggregationFunction::COUNT)
                        .interval("5m".to_string())
                        .type_(LogsComputeType::TIMESERIES)
                ],
            )
            .filter(
                LogsQueryFilter::new()
                    .from("now-15m".to_string())
                    .indexes(vec!["main".to_string()])
                    .query("*".to_string())
                    .to("now".to_string()),
            )
            .group_by(
                vec![
                    LogsGroupBy::new("host".to_string())
                        .missing(LogsGroupByMissing::LogsGroupByMissingString("miss".to_string()))
                        .sort(
                            LogsAggregateSort::new()
                                .aggregation(LogsAggregationFunction::PERCENTILE_90)
                                .metric("@duration".to_string())
                                .order(LogsSortOrder::ASCENDING)
                                .type_(LogsAggregateSortType::MEASURE),
                        )
                        .total(LogsGroupByTotal::LogsGroupByTotalString("recall".to_string()))
                ],
            );
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api.aggregate_logs(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
