// Create a new metric SLO object using bad events formula returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_service_level_objectives::ServiceLevelObjectivesAPI;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::SLOCountDefinition;
use datadog_api_client::datadogV1::model::SLOCountSpec;
use datadog_api_client::datadogV1::model::SLODataSourceQueryDefinition;
use datadog_api_client::datadogV1::model::SLOFormula;
use datadog_api_client::datadogV1::model::SLOSliSpec;
use datadog_api_client::datadogV1::model::SLOThreshold;
use datadog_api_client::datadogV1::model::SLOTimeframe;
use datadog_api_client::datadogV1::model::SLOType;
use datadog_api_client::datadogV1::model::ServiceLevelObjectiveRequest;

#[tokio::main]
async fn main() {
    let body = ServiceLevelObjectiveRequest::new(
        "Example-Service-Level-Objective".to_string(),
        vec![SLOThreshold::new(99.0, SLOTimeframe::SEVEN_DAYS)
            .target_display("99.0".to_string())
            .warning(99.5 as f64)
            .warning_display("99.5".to_string())],
        SLOType::METRIC,
    )
    .description(Some("Metric SLO using sli_specification".to_string()))
    .sli_specification(SLOSliSpec::SLOCountSpec(Box::new(SLOCountSpec::new(
        SLOCountDefinition::new(
            SLOFormula::new("query1 - query2".to_string()),
            vec![
                SLODataSourceQueryDefinition::FormulaAndFunctionMetricQueryDefinition(Box::new(
                    FormulaAndFunctionMetricQueryDefinition::new(
                        FormulaAndFunctionMetricDataSource::METRICS,
                        "query1".to_string(),
                        "sum:httpservice.hits{*}.as_count()".to_string(),
                    ),
                )),
                SLODataSourceQueryDefinition::FormulaAndFunctionMetricQueryDefinition(Box::new(
                    FormulaAndFunctionMetricQueryDefinition::new(
                        FormulaAndFunctionMetricDataSource::METRICS,
                        "query2".to_string(),
                        "sum:httpservice.errors{*}.as_count()".to_string(),
                    ),
                )),
            ],
        )
        .bad_events_formula(SLOFormula::new("query2".to_string())),
    ))))
    .tags(vec!["env:prod".to_string(), "type:count".to_string()])
    .target_threshold(99.0 as f64)
    .timeframe(SLOTimeframe::SEVEN_DAYS)
    .warning_threshold(99.5 as f64);
    let configuration = datadog::Configuration::new();
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api.create_slo(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
