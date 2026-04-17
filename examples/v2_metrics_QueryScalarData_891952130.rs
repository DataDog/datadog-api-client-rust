// Scalar cross product query with container data source returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::ContainerDataSource;
use datadog_api_client::datadogV2::model::ContainerScalarQuery;
use datadog_api_client::datadogV2::model::FormulaLimit;
use datadog_api_client::datadogV2::model::MetricsAggregator;
use datadog_api_client::datadogV2::model::QueryFormula;
use datadog_api_client::datadogV2::model::QuerySortOrder;
use datadog_api_client::datadogV2::model::ScalarFormulaQueryRequest;
use datadog_api_client::datadogV2::model::ScalarFormulaRequest;
use datadog_api_client::datadogV2::model::ScalarFormulaRequestAttributes;
use datadog_api_client::datadogV2::model::ScalarFormulaRequestType;
use datadog_api_client::datadogV2::model::ScalarQuery;

#[tokio::main]
async fn main() {
    let body = ScalarFormulaQueryRequest::new(ScalarFormulaRequest::new(
        ScalarFormulaRequestAttributes::new(
            1636625471000,
            vec![ScalarQuery::ContainerScalarQuery(Box::new(
                ContainerScalarQuery::new(
                    ContainerDataSource::CONTAINER,
                    "process.stat.container.cpu.system_pct".to_string(),
                    "a".to_string(),
                )
                .aggregator(MetricsAggregator::AVG)
                .limit(10)
                .sort(QuerySortOrder::DESC)
                .tag_filters(vec![]),
            ))],
            1636629071000,
        )
        .formulas(vec![QueryFormula::new("a".to_string())
            .limit(FormulaLimit::new().count(10).order(QuerySortOrder::DESC))]),
        ScalarFormulaRequestType::SCALAR_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.query_scalar_data(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
