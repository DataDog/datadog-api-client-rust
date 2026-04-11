// Scalar cross product query with slo data source returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::FormulaLimit;
use datadog_api_client::datadogV2::model::QueryFormula;
use datadog_api_client::datadogV2::model::QuerySortOrder;
use datadog_api_client::datadogV2::model::ScalarFormulaQueryRequest;
use datadog_api_client::datadogV2::model::ScalarFormulaRequest;
use datadog_api_client::datadogV2::model::ScalarFormulaRequestAttributes;
use datadog_api_client::datadogV2::model::ScalarFormulaRequestType;
use datadog_api_client::datadogV2::model::ScalarQuery;
use datadog_api_client::datadogV2::model::SloDataSource;
use datadog_api_client::datadogV2::model::SloQuery;
use datadog_api_client::datadogV2::model::SlosGroupMode;
use datadog_api_client::datadogV2::model::SlosMeasure;
use datadog_api_client::datadogV2::model::SlosQueryType;

#[tokio::main]
async fn main() {
    let body = ScalarFormulaQueryRequest::new(ScalarFormulaRequest::new(
        ScalarFormulaRequestAttributes::new(
            1636625471000,
            vec![ScalarQuery::SloQuery(Box::new(
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
