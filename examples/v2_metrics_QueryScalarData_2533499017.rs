// Scalar cross product query with apm_dependency_stats data source returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::ApmDependencyStatName;
use datadog_api_client::datadogV2::model::ApmDependencyStatsDataSource;
use datadog_api_client::datadogV2::model::ApmDependencyStatsQuery;
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
    let body =
        ScalarFormulaQueryRequest::new(
            ScalarFormulaRequest::new(
                ScalarFormulaRequestAttributes::new(
                    1636625471000,
                    vec![
                        ScalarQuery::ApmDependencyStatsQuery(
                            Box::new(
                                ApmDependencyStatsQuery::new(
                                    ApmDependencyStatsDataSource::APM_DEPENDENCY_STATS,
                                    "ci".to_string(),
                                    "a".to_string(),
                                    "cassandra.query".to_string(),
                                    "DELETE FROM monitor_history.monitor_state_change_history WHERE org_id = ? AND monitor_id IN ? AND group = ?".to_string(),
                                    "cassandra".to_string(),
                                    ApmDependencyStatName::AVG_DURATION,
                                )
                                    .primary_tag_name("datacenter".to_string())
                                    .primary_tag_value("edge-eu1.prod.dog".to_string()),
                            ),
                        )
                    ],
                    1636629071000,
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
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.query_scalar_data(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
