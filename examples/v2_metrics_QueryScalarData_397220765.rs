// Scalar cross product query with ci_tests data source returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::EventsAggregation;
use datadog_api_client::datadogV2::model::EventsCompute;
use datadog_api_client::datadogV2::model::EventsDataSource;
use datadog_api_client::datadogV2::model::EventsScalarQuery;
use datadog_api_client::datadogV2::model::EventsSearch;
use datadog_api_client::datadogV2::model::FormulaLimit;
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
            vec![ScalarQuery::EventsScalarQuery(Box::new(
                EventsScalarQuery::new(
                    EventsCompute::new(EventsAggregation::COUNT),
                    EventsDataSource::CI_TESTS,
                )
                .indexes(vec!["*".to_string()])
                .name("a".to_string())
                .search(EventsSearch::new().query("*".to_string())),
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
