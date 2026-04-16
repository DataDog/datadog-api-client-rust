// Timeseries cross product query with audit data source returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::EventsAggregation;
use datadog_api_client::datadogV2::model::EventsCompute;
use datadog_api_client::datadogV2::model::EventsDataSource;
use datadog_api_client::datadogV2::model::EventsSearch;
use datadog_api_client::datadogV2::model::EventsTimeseriesQuery;
use datadog_api_client::datadogV2::model::FormulaLimit;
use datadog_api_client::datadogV2::model::QueryFormula;
use datadog_api_client::datadogV2::model::QuerySortOrder;
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
            vec![TimeseriesQuery::EventsTimeseriesQuery(Box::new(
                EventsTimeseriesQuery::new(
                    EventsCompute::new(EventsAggregation::COUNT),
                    EventsDataSource::AUDIT,
                )
                .indexes(vec!["*".to_string()])
                .name("a".to_string())
                .search(EventsSearch::new().query("*".to_string())),
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
