// Timeseries cross product query with apm_resource_stats data source returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::ApmResourceStatName;
use datadog_api_client::datadogV2::model::ApmResourceStatsDataSource;
use datadog_api_client::datadogV2::model::ApmResourceStatsQuery;
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
            vec![TimeseriesQuery::ApmResourceStatsQuery(Box::new(
                ApmResourceStatsQuery::new(
                    ApmResourceStatsDataSource::APM_RESOURCE_STATS,
                    "staging".to_string(),
                    "a".to_string(),
                    "azure-bill-import".to_string(),
                    ApmResourceStatName::HITS,
                )
                .group_by(vec!["resource_name".to_string()])
                .operation_name("cassandra.query".to_string())
                .primary_tag_name("datacenter".to_string())
                .primary_tag_value("*".to_string()),
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
