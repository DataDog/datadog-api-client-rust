// Scalar cross product query returns "OK" response
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
        ScalarFormulaQueryRequest::new(
            ScalarFormulaRequest::new(
                ScalarFormulaRequestAttributes::new(
                    1671612804000,
                    vec![
                        ScalarQuery::MetricsScalarQuery(
                            Box::new(
                                MetricsScalarQuery::new(
                                    MetricsAggregator::AVG,
                                    MetricsDataSource::METRICS,
                                    "avg:system.cpu.user{*}".to_string(),
                                ).name("a".to_string()),
                            ),
                        )
                    ],
                    1671620004000,
                ).formulas(
                    vec![
                        QueryFormula::new(
                            "a".to_string(),
                        ).limit(FormulaLimit::new().count(10).order(QuerySortOrder::DESC))
                    ],
                ),
                ScalarFormulaRequestType::SCALAR_REQUEST,
            ),
        );
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryScalarData", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api.query_scalar_data(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
