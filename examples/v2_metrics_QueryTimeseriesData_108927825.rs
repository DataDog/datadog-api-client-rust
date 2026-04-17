// Timeseries cross product query with slo data source returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::FormulaLimit;
use datadog_api_client::datadogV2::model::QueryFormula;
use datadog_api_client::datadogV2::model::QuerySortOrder;
use datadog_api_client::datadogV2::model::SloDataSource;
use datadog_api_client::datadogV2::model::SloQuery;
use datadog_api_client::datadogV2::model::SlosGroupMode;
use datadog_api_client::datadogV2::model::SlosMeasure;
use datadog_api_client::datadogV2::model::SlosQueryType;
use datadog_api_client::datadogV2::model::TimeseriesFormulaQueryRequest;
use datadog_api_client::datadogV2::model::TimeseriesFormulaRequest;
use datadog_api_client::datadogV2::model::TimeseriesFormulaRequestAttributes;
use datadog_api_client::datadogV2::model::TimeseriesFormulaRequestType;
use datadog_api_client::datadogV2::model::TimeseriesQuery;

#[tokio::main]
async fn main() {
    let body = TimeseriesFormulaQueryRequest::new(TimeseriesFormulaRequest::new(
        TimeseriesFormulaRequestAttributes::new(
            1636625471000,
            vec![TimeseriesQuery::SloQuery(Box::new(
                SloQuery::new(
                    SloDataSource::SLO,
                    SlosMeasure::SLO_STATUS,
                    "12345678910".to_string(),
                )
                .additional_query_filters("*".to_string())
                .group_mode(SlosGroupMode::OVERALL)
                .name("a".to_string())
                .slo_query_type(SlosQueryType::METRIC),
            ))],
            1636629071000,
        )
        .formulas(vec![QueryFormula::new("a".to_string())
            .limit(FormulaLimit::new().count(10).order(QuerySortOrder::DESC))])
        .interval(5000),
        TimeseriesFormulaRequestType::TIMESERIES_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.query_timeseries_data(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
