// Query timeseries data across multiple products returns "OK" response
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
        TimeseriesFormulaQueryRequest::new(
            TimeseriesFormulaRequest::new(
                TimeseriesFormulaRequestAttributes::new(
                    1568899800000,
                    vec![
                        TimeseriesQuery::MetricsTimeseriesQuery(
                            Box::new(
                                MetricsTimeseriesQuery::new(
                                    MetricsDataSource::METRICS,
                                    "avg:system.cpu.user{*} by {env}".to_string(),
                                ),
                            ),
                        )
                    ],
                    1568923200000,
                )
                    .formulas(
                        vec![
                            QueryFormula::new(
                                "a+b".to_string(),
                            ).limit(FormulaLimit::new().count(10).order(QuerySortOrder::DESC))
                        ],
                    )
                    .interval(5000),
                TimeseriesFormulaRequestType::TIMESERIES_REQUEST,
            ),
        );
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryTimeseriesData", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api.query_timeseries_data(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
