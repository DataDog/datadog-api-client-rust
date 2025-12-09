// Create a new dashboard with a timeseries widget using formulas and functions
// metrics query with native semantic_mode
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricSemanticMode;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::TimeseriesWidgetDefinition;
use datadog_api_client::datadogV1::model::TimeseriesWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TimeseriesWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetDisplayType;
use datadog_api_client::datadogV1::model::WidgetFormula;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard with native semantic_mode".to_string(),
        vec![Widget::new(WidgetDefinition::TimeseriesWidgetDefinition(
            Box::new(TimeseriesWidgetDefinition::new(
                vec![TimeseriesWidgetRequest::new()
                    .display_type(WidgetDisplayType::LINE)
                    .formulas(vec![WidgetFormula::new("query1".to_string())])
                    .queries(vec![
                        FormulaAndFunctionQueryDefinition::FormulaAndFunctionMetricQueryDefinition(
                            Box::new(
                                FormulaAndFunctionMetricQueryDefinition::new(
                                    FormulaAndFunctionMetricDataSource::METRICS,
                                    "query1".to_string(),
                                    "avg:system.cpu.user{*}".to_string(),
                                )
                                .semantic_mode(FormulaAndFunctionMetricSemanticMode::NATIVE),
                            ),
                        ),
                    ])
                    .response_format(FormulaAndFunctionResponseFormat::TIMESERIES)],
                TimeseriesWidgetDefinitionType::TIMESERIES,
            )),
        ))],
    );
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
