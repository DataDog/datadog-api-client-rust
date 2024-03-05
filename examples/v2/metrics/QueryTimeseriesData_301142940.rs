// Timeseries cross product query returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        TimeseriesFormulaQueryRequest::new(
            TimeseriesFormulaRequest::new(
                TimeseriesFormulaRequestAttributes::new(
                    1671612804000,
                    vec![
                        TimeseriesQuery::MetricsTimeseriesQuery(
                            Box::new(
                                MetricsTimeseriesQuery::new(
                                    MetricsDataSource::METRICS,
                                    "avg:system.cpu.user{*}".to_string(),
                                ).name("a".to_string()),
                            ),
                        )
                    ],
                    1671620004000,
                )
                    .formulas(
                        vec![
                            QueryFormula::new(
                                "a".to_string(),
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
